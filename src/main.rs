use std::io::stdin;
use std::process::{exit, Command};

fn main() {
    let is_tmux = is_tmux_installed();
    if !is_tmux {
        println!("looks like u dont have tmux installed, or this program isnt detecting it in binary");
        exit(1)
    }

    println!("Welcome to mytmuxhelper\n");
    let (sessions, _isnt_empty) = list_sessions();
    let name_sess = name();
    let in_tmux = in_tmux();

    let found = sessions.iter().any(|s| s.trim() == name_sess.trim());
    let tmux_status = (found, in_tmux);

    match tmux_status {
        (true, false) => attach(&name_sess),
        (true, true) => change(&name_sess),
        (false, false) => new_attach(&name_sess),
        (false, true) => new_change(&name_sess),
    }
}

fn is_tmux_installed() -> bool {
    let cmd = Command::new("which").arg("tmux").output().expect("fail");
    let cmd_std = String::from_utf8_lossy(&cmd.stdout).trim().to_string();
    !cmd_std.is_empty() && cmd_std.len() < 50
}

fn list_sessions() -> (Vec<String>, bool) {
    let tmux_ls = Command::new("tmux").arg("ls").output();
    if tmux_ls.is_err() {
        return (Vec::new(), false);
    }
    let tmux_ls = tmux_ls.unwrap();
    let tmux_ls_std = String::from_utf8_lossy(&tmux_ls.stdout).trim().to_string();
    let sessions: Vec<String> = tmux_ls_std
        .lines()
        .filter_map(|line| {
            if let Some(idx) = line.find(':') {
                Some(line[..idx].trim().to_string())
            } else {
                None
            }
        })
        .collect();
    let isnt_empty = !sessions.is_empty();
    if !isnt_empty {
        println!("There are no tmux sessions\n");
    }
    else {
        let mut x = 0;
        println!("sessions are:");
        while x < sessions.len() {
            println!("{})  {}", x+1, sessions[x]);
            x += 1;
        }
        println!("\n");
    }
    (sessions, isnt_empty)
}

fn directory(linux_name: &str, is_zoxide: &bool) ->  String {
    let mut dic = String::new();

    // flush skipped since Write is not imported
    println!("the path of the directory: ");
    // flush skipped since Write is not imported
    stdin().read_line(&mut dic).unwrap();

    let dic = dic.trim().to_string();
    let dicc: Vec<_> = dic.split_whitespace().collect();

    if dic.is_empty(){
        return dic
    }
    if *is_zoxide {
        let cmd = Command::new("zoxide")
            .arg("query")
            .args(&dicc)
            .output()
            .expect("fail");
        let cmd_str = String::from_utf8_lossy(&cmd.stdout).trim().to_string();
        if cmd_str != "zoxide: no match found" {
            return cmd_str.to_string();
        }
    }

    if dic.starts_with('~') {
        let x = format!("/home/{}/{}", linux_name, &dic[1..]);
        return x.replace(" ", "/").trim().to_string();
    }
    dic
}

fn name() -> String {
    println!("write the name of your session");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();

    name
}

fn in_tmux() -> bool {
    std::env::var("TMUX").is_ok()
}

fn is_zoxide() -> bool {
    let cmd = Command::new("which").arg("zoxide").output().expect("fail");
    let cmd_std = String::from_utf8_lossy(&cmd.stdout).trim().to_string();
    !cmd_std.is_empty() && cmd_std.len() < 50
}

fn linux_name() -> String {
    let cmd = Command::new("whoami").output().expect("fail");
    String::from_utf8_lossy(&cmd.stdout).trim().to_string()
}

fn new_attach(name: &str) {
    let mut cmd = Command::new("tmux");
    let is_z = is_zoxide();
    let l_name = linux_name();
    let directory = directory(&l_name, &is_z);
    cmd.arg("new").arg("-s").arg(name.trim());
    if !directory.is_empty() {
        cmd.arg("-c").arg(directory);
    }
    cmd.status().expect("fail");
}

fn new_change(name: &str) {
    let mut cmd = Command::new("tmux");
    cmd.arg("new").arg("-d").arg("-s").arg(name.trim());
    let is_z = is_zoxide();
    let l_name = linux_name();
    let directory = directory(&l_name, &is_z);
    if !directory.is_empty() {
        cmd.arg("-c").arg(directory);
    }
    cmd.status().expect("fail");

    Command::new("tmux")
        .arg("switch-client")
        .arg("-t")
        .arg(name.trim())
        .status()
        .ok();
}

fn change(name: &str) {
    Command::new("tmux")
        .arg("switch-client")
        .arg("-t")
        .arg(name.trim())
        .status()
        .ok();
}

fn attach(name: &str) {
    Command::new("tmux")
        .arg("a")
        .arg("-t")
        .arg(name.trim())
        .status()
        .ok();
}

