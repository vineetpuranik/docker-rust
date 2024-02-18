use std::{io::Read, process::Stdio};

use anyhow::{Context, Result};

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() -> Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage!
    let args: Vec<_> = std::env::args().collect();
    let command = &args[3];
    let command_args = &args[4..];
    let output = std::process::Command::new(command)
        .args(command_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| {
            format!(
                "Tried to run '{}' with arguments {:?}",
                command, command_args
            )
        })?;
    
    output.stdout.map(|mut stdout| {
        let mut buf = String::new();
        stdout.read_to_string(&mut buf).unwrap();
        print!("{}", buf);
    });
    output.stderr.map(|mut stderr| {
        let mut buf = String::new();
        stderr.read_to_string(&mut buf).unwrap();
        eprint!("{}", buf);
    });


    Ok(())
}
