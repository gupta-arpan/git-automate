use std::process::{Command,exit};
use names::Generator;

fn add_commit_push() {
    let add_commmand = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to execute git add command!");

    if !add_commmand.status.success() {
        eprintln!("Error: failed to add files to git repo!");
        exit(1);
    }

    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(names_generator())
        .output()
        .expect("Failed to execute git commit command!");

    if !commit_command.status.success() {
        eprintln!("Error: Failed to commit files to git repo!");
        exit(1);
    }

    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("main")
        .output()
        .expect("Failed to execute git push command!");

    if !push_command.status.success() {
        eprintln!("Error: Failed to push changes to remote!");
        exit(1);
    }

    println!("Successfully added, commited and pushed all changes.");
}

fn names_generator() -> String {
    let mut generator = Generator::default();
    generator.next().unwrap()
}


fn main() {
    add_commit_push();
}
