use std::process::{Command, Stdio};
use std::io::{self, BufRead, BufReader};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

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

fn monitor_logs(receiver: Receiver<String>, keywords: &[&str], pid: Pid) {
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
            pause_process(pid);
        }
        println!("{}", line);
    }
}

fn pause_process(pid: Pid) {
    println!("Process paused. Press 'c' to continue...");
    // Send SIGSTOP to pause the process
    kill(pid, Signal::SIGSTOP).expect("Failed to pause process");

    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "c" {
            // Send SIGCONT to resume the process
            kill(pid, Signal::SIGCONT).expect("Failed to resume process");
            break;
        }
        println!("Invalid input. Press 'c' to continue...");
    }
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

    monitor_logs(receiver, &keywords, pid);
}
