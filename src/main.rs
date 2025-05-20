use std::io::stdin;
use std::process::Command;

fn main() {
    let mut is_session: bool = true;
    let output = Command::new("tmux")
        .arg("ls")
        .output()
        .expect("Failed to execute tmux ls");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let sessions: Vec<String> = output_str
        .lines()
        .filter_map(|line| line.split_once(':').map(|(name, _)| name.to_string()))
        .collect();
    println!("                            MyTmuxHelper");
    println!("<-------------------------------------------------------------------->");

    let mut mode = String::new();
    if sessions.len() >= 1 {
        print_sessions(&sessions);
    } 
    else{
        println!("no sessions created right not");
        is_session = false; 
    }
    if is_session == true {
        println!("modes --> change(c), new(n), new attach(na), attach(a), new change(nc)");
        println!("<-------------------------------------------------------------------->");
        stdin().read_line(&mut mode).unwrap();
        let mode = mode.trim();
        match mode {
            "c" | "change" => change(),
            "n" | "new" => new(),
            "na" | "new attach" => new_attach(),
            "a" | "attach" => attach(),
            "nc" | "new change" => new_change(),
            _ => println!("Unknown mode"),
        }
    } 
    else {
        println!("modes -->  new(n), new attach(na)");

        println!("<-------------------------------------------------------------------->");

        stdin().read_line(&mut mode).unwrap();
        let mode = mode.trim();
        match mode {
            "n" | "new" => new(),
            "na" | "new attach" => new_attach(),
            _ => println!("Unknown mode"),
        }
    }
    println!("<-------------------------------------------------------------------->");

}

fn print_sessions(sessions: &Vec<String>) {
    println!("Available tmux sessions:");
    let mut m: usize = 0;
    while m < sessions.len(){
        println!("{}) --> {}", m+1, sessions[m]);
        m += 1;
    }
}

fn change() {
    let mut name = String::new();
    println!("<-------------------------------------------------------------------->");
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

fn new() {
    let mut name = String::new();
    println!("<-------------------------------------------------------------------->");
 
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

fn new_attach() {
    println!("<-------------------------------------------------------------------->");
 
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

fn attach() {
    println!("<-------------------------------------------------------------------->");

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

fn new_change() {
    println!("<-------------------------------------------------------------------->");

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

