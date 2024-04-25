use std::process::{Command, exit};


fn update_commit_push() {
    // Command 1: Add all files recursively to git repo
    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to execute git add command");

    if !add_command.status.success() {
        eprintln!("Error: Failed to add files to the git repo.");
        exit(1);
    }

    // Get the current executable path
    let current_exe_path = std::env::current_exe()
        .expect("Failed to get current executable path");

// Get the file name from the path
    let file_name = current_exe_path
        .file_name()
        .expect("Failed to get file name")
        .to_str()
        .expect("Failed to convert file name to string");


    // Command 2: Commit all changes with file name as commit message
    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(file_name)
        .output()
        .expect("Failed to execute git commit command");

    if !commit_command.status.success() {
        eprintln!("Error: Failed to commit changes. Git error: {:?}", commit_command.stderr);
        exit(1);
    }

    // Command 3: Push to remote (origin main)
    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("Failed to execute git push command");

    if !push_command.status.success() {
        eprintln!("Error: Failed to push changes to remote.");
        exit(1);
    }

    println!("Successfully added, committed, and pushed changes!");
}

fn main() {
    update_commit_push();
}

