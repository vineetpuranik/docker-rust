use anyhow::{Context, Ok, Result};
use std::os::unix::fs::chroot;
use std::fs;


//created connection between code crafters and my github repo
//create an isolated path under a new folder called temp in the current directory
const ISOLATED_PATH: &str = "./temp";

// Usage: your_docker.sh run <image> <command> <arg> <arg2> ...
fn main() -> Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage!

    // create an empty temporary directory based on the isolated path
    // in the directory create the path /dev/null 
    let create_dir_path = format!("{}{}", ISOLATED_PATH, "/dev/null");
    fs::create_dir_all(create_dir_path)?;
    
    // create the /usr/local/bin directory in our temo directory
    let create_bin_directory = format!("{}{}", ISOLATED_PATH, "/usr/local/bin/");
    fs::create_dir_all(create_bin_directory)?;
    
    //capture user input for command and command args
    let args: Vec<_> = std::env::args().collect();
    let command = &args[3];
    let command_args = &args[4..];

    //copy binary to current working directory
    fs::copy(command, format!("{}{}", ISOLATED_PATH, command)).context("Failed to copy")?;

    //perform chroot operation
    chroot(ISOLATED_PATH)?;
    
    //change the current working directory
    std::env::set_current_dir(ISOLATED_PATH)?;

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
