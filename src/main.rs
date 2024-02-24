use anyhow::{Context, Ok, Result};
use std::{fs, path::Path};
use std::os::unix::fs::chroot;


// Usage: your_docker.sh run <image> <command> <arg> <arg2> ...
fn main() -> Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage!

    // create an empty temporary directory
    // create an empty dev null in this directory
    fs::create_dir_all("/temp/dev/null")?;
    
    //perform chroot operation
    chroot("/temp")?;
    
    //change the current working directory
    std::env::set_current_dir("/temp");
    
    // chroot into the directory while executing the command and also copy over the binary
    let args: Vec<_> = std::env::args().collect();
    let command = &args[3];
    let command_args = &args[4..];

    //copy binary to current working directory
    let copy_destination = "/temp".to_owned() + command.strip_prefix("/").unwrap();
    fs::copy(command, copy_destination).context("Failed to copy")?;

    let output = std::process::Command::new(command)
        .args(command_args)
        .output()
        .with_context(|| {
            format!(
                "Tried to run '{}' with arguments {:?}",
                command, command_args
            )
        })?;
    
    
    //Wire up stdout to parent process
    let std_out = std::str::from_utf8(&output.stdout)?;
    print!("{}", std_out);

    //Wire up stderr to parent process    
    let std_err = std::str::from_utf8(&output.stderr)?;
    eprint!("{}", std_err);

    //Wait for the child process to exit and check the exit status
    let exit_code = output.status.code();

    match exit_code {
        Some(code) => std::process::exit(code),
        None => eprint!("No exit code returned")
    }
    
    Ok(())

}
