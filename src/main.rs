use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::signal;
use wiktionary_cli::get_word; // Ensure this is correctly imported

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Spawn a task to handle SIGINT (Ctrl+C)
    tokio::spawn(async {
        // Wait for Ctrl+C
        if signal::ctrl_c().await.is_ok() {
            // Write to stderr to ensure immediate display
            eprintln!("\nSIGINT received, exiting");
            std::process::exit(0);
        }
    });

    // Create a buffered reader for stdin
    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    loop {
        // Display the prompt
        if let Err(e) = io::stdout().write_all(b"> ").await {
            eprintln!("Failed to write prompt: {}", e);
            break;
        }
        if let Err(e) = io::stdout().flush().await {
            eprintln!("Failed to flush stdout: {}", e);
            break;
        }

        // Read a line of input
        match lines.next_line().await {
            Ok(Some(line)) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue; // Skip empty lines
                }

                // Fetch the word asynchronously
                match get_word(trimmed).await {
                    Ok(word) => println!("{:?}", word),
                    Err(e) => eprintln!("Error fetching word: {}", e),
                }
            }
            Ok(None) => {
                // EOF reached (e.g., Ctrl+D), exit gracefully
                println!("\nEOF received, exiting");
                break;
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
    }

    Ok(())
}
