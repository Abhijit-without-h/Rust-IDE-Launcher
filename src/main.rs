use std::process::Command;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    loop {
        println!("Which program would you like to launch?");
        println!("1. Cursor");
        println!("2. Visual Studio Code");
        println!("3. IntelliJ IDEA");
        println!("4. Exit");
        print!("Enter your choice (1, 2, 3, or 4): ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => launch_program("cursor"),
            "2" => launch_program("code"),
            "3" => launch_program("idea"),
            "4" => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }

    Ok(())
}

fn launch_program(program: &str) {
    let status = Command::new(program).status();

    match status {
        Ok(_) => println!("{} launched successfully.", program),
        Err(e) => println!("Failed to launch {}: {}", program, e),
    }
}