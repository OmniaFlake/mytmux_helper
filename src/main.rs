use std::io::stdin;
use std::process::Command;

fn main() {
    let mut mode = String::new();
    println!("modes --> change(c), new(n), new attach(na), attach(a), new change(nc)");
    stdin().read_line(&mut mode).unwrap();
    let mode = mode.trim();

    let output = Command::new("tmux")
        .arg("ls")
        .output()
        .expect("Failed to execute tmux ls");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let sessions: Vec<String> = output_str
        .lines()
        .filter_map(|line| line.split_once(':').map(|(name, _)| name.to_string()))
        .collect();

    match mode {
        "c" | "change" => change(&sessions),
        "n" | "new" => new(&sessions),
        "na" | "new attach" => new_attach(&sessions),
        "a" | "attach" => attach(&sessions),
        "nc" | "new change" => new_change(&sessions),
        _ => println!("Unknown mode"),
    }
}

fn print_sessions(sessions: &Vec<String>) {
    println!("Available tmux sessions:");
    for session in sessions {
        println!("{}", session);
    }
}

fn change(sessions: &Vec<String>) {
    print_sessions(sessions);
    let mut name = String::new();
    println!("Session name to switch to:");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    Command::new("tmux")
        .arg("switch-client")
        .arg("-t")
        .arg(name)
        .status()
        .expect("Failed to change session");
}

fn new(sessions: &Vec<String>) {
    print_sessions(sessions);
    let mut name = String::new();
    println!("New session name:");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    Command::new("tmux")
        .arg("new")
        .arg("-d")
        .arg("-s")
        .arg(name)
        .status()
        .expect("Failed to start new session");
}

fn new_attach(sessions: &Vec<String>) {
    print_sessions(sessions);

    let mut name = String::new();
    println!("New session name:");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    Command::new("tmux")
        .arg("new")
        .arg("-s")
        .arg(name)
        .status()
        .expect("Failed to start and attach session");
}

fn attach(sessions: &Vec<String>) {
    print_sessions(sessions);
    let mut name = String::new();
    println!("Session name to attach to:");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    Command::new("tmux")
        .arg("attach")
        .arg("-t")
        .arg(name)
        .status()
        .expect("Failed to attach to session");
}

fn new_change(sessions: &Vec<String>) {
    print_sessions(sessions);
    let mut name = String::new();
    println!("New session name:");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    Command::new("tmux")
        .arg("new")
        .arg("-d")
        .arg("-s")
        .arg(name)
        .status()
        .expect("Failed to create session");

    Command::new("tmux")
        .arg("switch-client")
        .arg("-t")
        .arg(name)
        .status()
        .expect("Failed to switch to session");
}

