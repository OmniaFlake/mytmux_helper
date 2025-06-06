use std::io::{stdin, Write};
use std::process::Command;

/// Utility: Expand ~ to /home/<user> in directory paths.
/// Utility: Detect if we are running inside a tmux session.
fn expand_home(dic: &str, linux_name: &str, is_zz: bool) -> String {
    if is_zz {
        let x = Command::new("zoxide").arg("query").arg(dic).output().expect("lol");
        let xo = String::from_utf8_lossy(&x.stdout).trim().to_string();
        if xo != "zoxide : no match found"{
            return xo
        }
    }
    if dic.starts_with('~') {
        let og =  format!("/home/{}/{}", linux_name.trim(), &dic[1..]);
        return og.replace(" ", "/");
    } else {
        dic.to_string().replace(" ", "/")
    }
}
fn in_tmux() -> bool {
    std::env::var("TMUX").map_or(false, |v| !v.trim().is_empty())
}

/// Utility: Print current tmux sessions.
fn print_sessions(sessions: &[String]) {
    println!("Available tmux sessions:");
    for (i, session) in sessions.iter().enumerate() {
        println!("{}) --> {}", i + 1, session);
    }
}

/// Action: Switch to another session.
fn change() {
    let mut name = String::new();
    print!("Session name to switch to: ");
    std::io::stdout().flush().ok();
    stdin().read_line(&mut name).unwrap();
    let name = name.trim();
    Command::new("tmux")
        .arg("switch-client")
        .arg("-t")
        .arg(name)
        .status()
        .expect("Failed to change session");
}

/// Action: Create a new session (detached).
fn new(linux_name: &str) {
    let (name, dic) = get_session_name_and_dir(linux_name);
    let mut cmd = Command::new("tmux");
    cmd.arg("new").arg("-d").arg("-s").arg(&name);
    if !dic.is_empty() {
        cmd.arg("-c").arg(dic);
    }
    cmd.status().expect("Failed to start new session");
}

/// Action: Create a new session and attach.
fn new_attach(linux_name: &str) {
    let (name, dic) = get_session_name_and_dir(linux_name);
    let mut cmd = Command::new("tmux");
    cmd.arg("new").arg("-s").arg(&name);
    if !dic.is_empty() {
        cmd.arg("-c").arg(dic);
    }
    cmd.status().expect("Failed to start new session");
}

/// Action: Attach to an existing session.
fn attach() {
    let mut name = String::new();
    print!("Session name to attach to: ");
    std::io::stdout().flush().ok();
    stdin().read_line(&mut name).unwrap();
    let name = name.trim();
    Command::new("tmux")
        .arg("attach")
        .arg("-t")
        .arg(name)
        .status()
        .expect("Failed to attach to session");
}

/// Action: Create a new session and switch to it.
fn new_change(linux_name: &str) {
    let (name, dic) = get_session_name_and_dir(linux_name);
    let mut cmd = Command::new("tmux");
    cmd.arg("new").arg("-d").arg("-s").arg(&name);
    if !dic.is_empty() {
        cmd.arg("-c").arg(dic);
    }
    cmd.status().expect("Failed to start new session");

    Command::new("tmux")
        .arg("switch-client")
        .arg("-t")
        .arg(name)
        .status()
        .expect("Failed to switch to session");
}

/// Utility: Prompt for session name and directory path.
fn get_session_name_and_dir(linux_name: &str) -> (String, String) {
    let mut name = String::new();
    let mut dic = String::new();
    let is_z: bool = is_zoxide();
    print!("New session name: ");
    std::io::stdout().flush().ok();
    stdin().read_line(&mut name).unwrap();
    print!("Write directory path, or leave it empty for same path: ");
    std::io::stdout().flush().ok();
    stdin().read_line(&mut dic).unwrap();
    let name = name.trim().to_string();
    let dic = expand_home(dic.trim(), linux_name, is_z);
    (name, dic)
}

fn is_zoxide()-> bool {
    let x = Command::new("which")
        .arg("zoxide")
        .output()
        .expect("error lo");
    if String::from_utf8_lossy(&x.stdout).trim().to_string() == "/run/current-system/sw/bin/zoxide" {
        return true
    }
    false
}

fn main() {
    let output = Command::new("tmux")
        .arg("ls")
        .output()
        .expect("Failed to execute tmux ls");
    let name = Command::new("whoami")
        .output()
        .expect("Can't get the name");
    let linux_name = String::from_utf8_lossy(&name.stdout).trim().to_string();
    let output_str = String::from_utf8_lossy(&output.stdout);
    let sessions: Vec<String> = output_str
        .lines()
        .filter_map(|line| line.split_once(':').map(|(name, _)| name.to_string()))
        .collect();

    println!("                            MyTmuxHelper");
    println!("<-------------------------------------------------------------------->");
    let in_tmux = in_tmux();
    let is_session = !sessions.is_empty();
    if is_session {
        print_sessions(&sessions);
    } else {
        println!("No sessions created right now");
    }

    let mut mode = String::new();
    let show_modes = |modes: &[(&str, &str)]| {
        let descs: Vec<String> = modes.iter().map(|(d, k)| format!("{}({})", d, k)).collect();
        println!("modes --> {}", descs.join(", "));
        println!("<-------------------------------------------------------------------->");
    };

    // Mode selection logic
    if is_session && sessions.len() > 1 {
        if in_tmux {
            show_modes(&[("change", "c"), ("new", "n"), ("new change", "nc")]);
            stdin().read_line(&mut mode).unwrap();
            match mode.trim() {
                "c" | "change" => change(),
                "n" | "new" => new(&linux_name),
                "nc" | "new change" => new_change(&linux_name),
                _ => println!("Unknown mode"),
            }
        } else {
            show_modes(&[("new", "n"), ("new attach", "na"), ("attach", "a"), ("new change", "nc"), ("change", "c")]);
            stdin().read_line(&mut mode).unwrap();
            match mode.trim() {
                "n" | "new" => new(&linux_name),
                "na" | "new attach" => new_attach(&linux_name),
                "a" | "attach" => attach(),
                "nc" | "new change" => new_change(&linux_name),
                "c" | "change" => change(),
                _ => println!("Unknown mode"),
            }
        }
    } else if is_session {
        if in_tmux {
            show_modes(&[("new", "n"), ("new change", "nc")]);
            stdin().read_line(&mut mode).unwrap();
            match mode.trim() {
                "n" | "new" => new(&linux_name),
                "nc" | "new change" => new_change(&linux_name),
                _ => println!("Unknown mode"),
            }
        } else {
            show_modes(&[("new", "n"), ("new attach", "na"), ("attach", "a")]);
            stdin().read_line(&mut mode).unwrap();
            match mode.trim() {
                "n" | "new" => new(&linux_name),
                "na" | "new attach" => new_attach(&linux_name),
                "a" | "attach" => attach(),
                _ => println!("Unknown mode"),
            }
        }
    } else {
        show_modes(&[("new", "n"), ("new attach", "na")]);
        stdin().read_line(&mut mode).unwrap();
        match mode.trim() {
            "n" | "new" => new(&linux_name),
            "na" | "new attach" => new_attach(&linux_name),
            _ => println!("Unknown mode"),
        }
    }

    println!("<-------------------------------------------------------------------->");
}

