use std::process::{Command, Stdio};
use std::io::{self, BufRead, BufReader};
use std::sync::mpsc::{channel, Receiver};
use std::thread;

fn execute_subcommand(command: &str, args: &[&str]) -> Receiver<String> {
    let (sender, receiver) = channel();
    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start subcommand");

    let stdout = child.stdout.take().expect("Failed to capture stdout");

    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                sender.send(line).expect("Failed to send log line");
            }
        }
    });

    receiver
}

fn monitor_logs(receiver: Receiver<String>, keywords: &[&str]) {
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
            pause_process();
        }
        println!("{}", line);
    }
}

fn pause_process() {
    println!("Process paused. Press 'c' to continue...");
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "c" {
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

    let receiver = execute_subcommand(command, &command_args);

    monitor_logs(receiver, &keywords);
    println!("Hello, world!");
}
