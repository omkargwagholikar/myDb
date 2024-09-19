use std::process::{Command, Stdio};
use std::io::Write;

// static test_db_name:String = String::from("test.db");

fn run_script(commands: Vec<&str>) -> Vec<String> {
    let mut child = Command::new("./target/debug/dbV1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start child process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        for command in commands {
            stdin.write_all(command.as_bytes()).expect("Failed to write to stdin");
            stdin.write_all(b"\n").expect("Failed to write newline to stdin");
        }
    }

    let output = child.wait_with_output().expect("Failed to read stdout");
    let raw_output = String::from_utf8_lossy(&output.stdout);

    raw_output.split('\n').map(|s| s.to_string()).collect()
}

fn drop_db() {
    let status = Command::new("rm")
        .arg("test.db")
        .status()
        .expect("Failed to execute rm command");

    if status.success() {
        println!("Database 'test.db' dropped successfully.");
    } else {
        println!("Failed to drop the database.");
    }
}

fn insert() {
    let result = run_script(vec![
        "insert 1 user1 person1@example.com",
        "select",
        ".exit"
    ]);
    
    assert_eq!(result, vec![
        "dbv1 > Executed. ",
        "dbv1 > Id\tUsername\tEmail", 
        "1\tuser1\t\tperson1@example.com", 
        "Executed. ",
        "dbv1 > "
    ]);
}

fn check() {
    let result = run_script(vec![
        "select",
        ".exit"
    ]);
    assert_eq!(result, vec![
        "dbv1 > Id\tUsername\tEmail", 
        "1\tuser1\t\tperson1@example.com", 
        "Executed. ",
        "dbv1 > "
    ]);

}

# [test]
fn verify_persistence() {
    drop_db();
    insert();
    check();
    drop_db();
}