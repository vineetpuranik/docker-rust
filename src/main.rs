use anyhow::{Context, Ok, Result};
use std::os::unix::fs::chroot;
use std::fs;


//created connection between code crafters and my github repo
//create an isolated path under a new folder called temp in the current directory.This will represent the sandboxed environment for our isolated execution.
//create dev/null in the temp directory based on the instructions
//create /usr/local/bin in the temp directory
//read the arguments and copy the contents of the command binary supplied by the user to our temp directory
//chroot in our temp directory to created an isolated environment
//change the current working directory to the temp working directory
//creating a new process using the input command and this will use the temp directory as the starting environment for the process
//wire output and error from the new process to our parent
//get the exit code of the new process and make sure the parent also exists with the same exit code.



//Sandboxing process : helps guarding against malicious actitity by restricting the spawned process's executable file access to only the files within the sandboxed process
//Chroot
//fs create directory
//parse user arguments
//copy contents of one file to the other file using fs copy
//change working directory set_current_dir
//spawn a child process and provide the command for the child process
//wire up child process output and error to parent.
//PID namespaces to ensure the spawned process is capable of viewing only itself.
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
    
    // create the /usr/local/bin directory in our temp directory
    let create_bin_directory = format!("{}{}", ISOLATED_PATH, "/usr/local/bin/");
    fs::create_dir_all(create_bin_directory)?;
    
    //capture user input for command and command args
    let args: Vec<_> = std::env::args().collect();
    let command = &args[3];
    let command_args = &args[4..];

    //copy the contents of the binary provided by the user to our current working directory
    fs::copy(command, format!("{}{}", ISOLATED_PATH, command)).context("Failed to copy")?;

    //perform chroot operation
    chroot(ISOLATED_PATH)?;
    
    //change the current working directory
    std::env::set_current_dir(ISOLATED_PATH)?;

    // implement process isolation
    unsafe {
        #[cfg(target_os = "linux")]
        libc::unshare(libc::CLONE_NEWPID);
    }

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
