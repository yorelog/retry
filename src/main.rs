use std::process::{Command,Stdio};
use std::env;
use std::thread;
use std::time::Duration;

fn main(){
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Retry command execution until it succeeds or max retry count is exceeded.\n");
        println!("Usage: retry <command>\n\n");
        println!("Example: retry git pull\n");
        std::process::exit(1);
    }

    let cmd = &args[1..].join(" ");
    let max_retry_count = 30;
    let mut retry_count = 0;
    let mut last_error_msg = String::new();

    let exec_cmd = if cfg!(windows) {"cmd"} else {"sh"};
    let exec_arg = if cfg!(windows) {"/C"} else {"-c"};
    loop {
        let current_dir = env::current_dir().unwrap();
        println!("Current dir: {:?}",current_dir);

        let output = Command::new(exec_cmd)
            .arg(exec_arg)
            .arg(cmd)
            .current_dir(&current_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();
        match output {
            Ok(output) =>{
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                    println!("Command executed sucessfully!\nOutput:\n{}",stdout);
                    break;
                } else {
                    last_error_msg = String::from_utf8_lossy(&output.stderr).to_string();
                    println!("Error occurred during command execution:\n{}",last_error_msg);

                }
            }
            Err(e) => {
                last_error_msg = e.to_string();
                println!("Error occurred during command execution:\n{}",last_error_msg);
            }
        }
        if retry_count >= max_retry_count {
            println!("Max retry count execeeded...");
            break;
        }
        retry_count +=1;
        println!("Retry {} times",retry_count);
        thread::sleep(Duration::from_secs(1));
    }
    if last_error_msg.is_empty() {
        println!("Command executed successfully!");
    }

}