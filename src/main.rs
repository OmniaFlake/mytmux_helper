use std::process::Command;
use std::io::stdin;

fn main() {
    let mut mode = String::new();
    println!("modes --> change(c), new(n), new attach(na), attach(a)");
    stdin().read_line(&mut mode).unwrap();
    let mode = mode.trim();
    match mode {
        "c" | "change" => change(),
        "n" | "new" => new(),
        "na" | "new attach" => new_attach(),
        "a" | "attach" => attach(),
        _ => println!("Unknown mode"),
    }
}

fn change() {
    let output = Command::new("tmux")
        .arg("ls")
        .output()
        .expect("Failed to execute tmux ls");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut sessions = Vec::new();

    for line in output_str.lines() {
        if let Some((session_name, _)) = line.split_once(':') {
            sessions.push(session_name.to_string());
        }
    }

    println!("Available tmux sessions:");
    for session in &sessions {
        println!("{}", session);
    }

    let mut name = String::new();
    println!("Session name to switch to:");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    Command::new("tmux")
        .arg("switchc") // Probably meant "switch-client", adjust as needed
        .arg("-t")
        .arg(name)
        .status()
        .expect("Failed to change session");
}

fn new() {
    let mut name1 = String::new();
    println!("New session name:");
    stdin().read_line(&mut name1).unwrap();
    let name1 = name1.trim();

    Command::new("tmux")
        .arg("new")
        .arg("-d")
        .arg("-s")
        .arg(name1)
        .status()
        .expect("Failed to start new session");
}
fn new_attach() {
    let mut name1 = String::new();
    println!("New session name:");
    stdin().read_line(&mut name1).unwrap();
    let name1 = name1.trim();

    Command::new("tmux")
        .arg("new")
        .arg("-s")
        .arg(name1)
        .status()
        .expect("Failed to start new session");
}
fn attach() {
    let output = Command::new("tmux")
        .arg("ls")
        .output()
        .expect("Failed to execute tmux ls");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut sessions = Vec::new();

    for line in output_str.lines() {
        if let Some((session_name, _)) = line.split_once(':') {
            sessions.push(session_name.to_string());
        }
    }

    println!("Available tmux sessions:");
    for session in &sessions {
        println!("{}", session);
    }

    let mut name = String::new();
    println!("Session name to attach to:");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    Command::new("tmux")
        .arg("a") 
        .arg("-t")
        .arg(name)
        .status()
        .expect("Failed to attach session");
}
