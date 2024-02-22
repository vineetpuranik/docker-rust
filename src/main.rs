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
    print!("{}", std_err);

    //Wait for the child process to exit and check the exit status
    let exit_code = output.status.code();

    match exit_code {
        Some(code) => std::process::exit(code),
        None => eprint!("No exit code returned")
    }

    Ok(())

}
