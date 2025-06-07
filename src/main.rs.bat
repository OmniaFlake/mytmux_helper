use std::{io::stdin, process::{exit, Command}};

fn main() {
    let is_tmux = is_tmux_installed();
    if !is_tmux {
        println!("looks like u dont have tmux installed, or this program isnt detecting in binary");
        exit(1)
    }

    let is_z: bool = is_zoxide();
    let linux_name = linux_name();
    let name = String::new();
    let (sessions, isnt_empty) = list_sessions();
    let (directory, name) = directory_name(linux_name, is_z);

    let tmux_status = (sessions.contains(name), in_tmux)

    match tmux_status {
        (true, false) : attach(),
        (true, true) : change(),
        (false, false): new_attach(),
        (false, true) : new_change()
    }
}

fn is_tmux_installed() -> bool{
    let cmd = Command::new("which").arg("tmux").output().expect("fail");
    let cmd_std = String::from_utf8_lossy(&cmd.stdout).trim().to_string();
    if cmd_std.len() < 50 && cmd_std.len() != 0{ 
        return true
    }
    return false
}

fn list_sessions () -> (Vec<String>, bool) {
    let tmux_ls = Command::new("tmux")
        .arg("ls")
        .output()
        .expect("why didnt it work ");

    let tmux_ls_std = String::from_utf8_lossy(&tmux_ls.stdout).trim().to_string();
    let sessions: Vec<String> = tmux_ls_std.lines()
        .filter_map(|line| line.split_once(":").map(|(name, _)| name.to_string()))
        .collect();
    if sessions.len() > 0 {
        return (sessions, true)
    }
    return (sessions, false)
}

fn directory_name(linux_name: str, iss_zoxide: bool) -> (String, String){

    let mut name = String::new();
    let mut dic = String::new();
    
    println!("name of the sesssion :");
    stdin().read_line(& mut name).unwrap();
    
    println!("the path of the directory: ");
    stdin().read_line(& mut dic).unwrap();
    
    if iss_zoxide{
        let cmd = Command::new("zoxide").arg("query").arg(dic).output().expect("failed");
        let cmd_std = String::from_utf8_lossy(&cmd.stdout);
    
        if cmd != "zoxide: no match found" {
            (return cmd_std, name)
        }

    }
    if dic.starts_with("~") {
        let x = format!("/home/{}/{}",&linux_name,  &dic[1..])
        return (x.replace(" ", "/").trim().to_string() , name)
    } 
        return (dic.to_string(), name);
}

fn in_tmux() -> bool {
    let cmd = Command::new("echo").arg("$TERM").output().expect("fail");
    let cmd_std = String::from_utf8_lossy(&cmd.stdout);

    if cmd_std[0..4] == "tmux" {
        true;
    }
    false
}

fn is_zoxide() -> bool{
    let cmd = Command::new("which").arg("zoxide").output().expect("fail");
    let cmd_std = String::from_utf8_lossy(&cmd.stdout).trim().to_string();

    if cmd_std.len() < 50 {
        true;
    }
    false
}

fn linux_name() -> String {
    let cmd =  Command::new("whoami").output().expect("fail");
    return String::from_utf8_lossy(&cmd.stdout).trim().to_string()

}
fn new_attach(name: str, directory: str) {
    let mut cmd = Command::new("tmux")
    .arg("new")
    .arg("-s")
    .arg(name);
    if directory.is_empty() == false {
        cmd.arg(-c).arg(dic);
    }
    cmd.status().expect("fail");
}

fn new_change(name: str, directory: str) {
    let mut cmd = Command::new("tmux")
    .arg("new")
    .arg("-d")
    .arg("-s")
    .arg(name);
    if directory.is_empty() == false {
        cmd.arg("-c").arg(dic);
    }
    cmd.status().expect("fail");

    Command::new("tmux").arg("switch-client").arg("-t").arg(name).status();
}

fn change(name: str) {
    Command::new("tmux").arg("switch-client").arg("-t").arg(name).status();
}

fn attach(name: str ){
    Command::new("tmux").arg("a").arg("-t").arg(name).status();
}
