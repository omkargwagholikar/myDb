use std::process::{Command, Stdio};
use std::io::Write;

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

#[test]
fn inserts_and_retrieves_a_row() {
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

#[test]
fn allows_inserting_strings_that_are_the_maximum_length() {
    let long_username = "a".repeat(32);
    let long_email = "a".repeat(255);
    let result = run_script(vec![
        &format!("insert 1 {} {}", long_username, long_email),
        "select",
        ".exit",
    ]);
    assert_eq!(result, vec![
        "dbv1 > Executed. ",
        "dbv1 > Id\tUsername\tEmail", 
        &format!("1\t{}\t\t{}", long_username, long_email),
        "Executed. ",
        "dbv1 > ",
    ]);
}

#[test]
fn prints_error_message_if_strings_are_too_long() {
    let long_username = "a".repeat(33);
    let long_email = "a".repeat(256);
    let result = run_script(vec![
        &format!("insert 1 {} {}", long_username, long_email),
        "select",
        ".exit",
    ]);
    assert_eq!(result, vec![
        "dbv1 > String is too long.",
        "dbv1 > Id\tUsername\tEmail",
        "Executed. ",
        "dbv1 > ",
    ]);
}

#[test]
fn prints_error_message_if_id_is_negative() {
    let result = run_script(vec![
        "insert -1 cstack foo@bar.com",
        "select",
        ".exit",
    ]);
    assert_eq!(result, vec![
        "dbv1 > ID must be positive.",
        "dbv1 > Id\tUsername\tEmail",
        "Executed. ",
        "dbv1 > ",
    ]);
}