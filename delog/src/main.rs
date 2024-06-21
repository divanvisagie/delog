use std::process::{Command, Stdio};
use std::io::{self, BufRead, BufReader, Write};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use sysinfo::System;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::children::get_child_pids;

mod children;

fn execute_subcommand(command: &str, args: &[&str]) -> (Receiver<String>, Pid) {
    let (sender, receiver) = channel();
    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start subcommand");

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let pid = Pid::from_raw(child.id() as i32);

    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                sender.send(line).expect("Failed to send log line");
            }
        }
    });

    (receiver, pid)
}

fn monitor_logs(system: &System, receiver: Receiver<String>, keywords: &[&str], pid: Pid) {
    for line in receiver {
        let mut keyword_found = false;
        for &keyword in keywords {
            if line.contains(keyword) {
                keyword_found = true;
                break;
            }
        }
        if keyword_found {
            println!("Breakpoint: {}", line);
            pause_process(system, pid);
            println!("");
            continue;
        }
        println!("{}", line);
    }
}

fn pause_process(system: &System, pid: Pid) {
    println!("Process paused. Press 'c' to continue...");
    
    // Pause the main process
    kill(pid, Signal::SIGSTOP).expect("Failed to pause main process");

    // Pause forked child processes
    let child_pids = get_child_pids(system, pid);
    for child_pid in &child_pids {
        kill(*child_pid, Signal::SIGSTOP).expect("Failed to pause child process");
    }

    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            termion::event::Key::Char('c') => {
                // Resume the main process
                kill(pid, Signal::SIGCONT).expect("Failed to resume main process");

                // Resume forked child processes
                for child_pid in &child_pids {
                    kill(*child_pid, Signal::SIGCONT).expect("Failed to resume child process");
                }
                break;
            }
            _ => {}
        }
    }

    println!("Continuing execution...");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: delog <command> [args...]");
        std::process::exit(1);
    }

    let command = &args[1];
    let command_args: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();
    let keywords = ["BREAKPOINT", "BREAK"];

    let (receiver, pid) = execute_subcommand(command, &command_args);
    let system = System::new_all();
    monitor_logs(&system, receiver, &keywords, pid);
}
