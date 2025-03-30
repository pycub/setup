use anyhow::{Context, Result};
use console::Term;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn run_command(program: &str, args: &[&str]) -> Result<()> {
    let term = Term::stdout();
    let args_str = args.join(" ");

    term.write_line(&format!("$ {} {}", program, args_str))?;

    let mut child = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to execute command: {} {}", program, args_str))?;

    let stdout = BufReader::new(child.stdout.take().unwrap());
    let stderr = BufReader::new(child.stderr.take().unwrap());

    // Process stdout in a separate thread
    let thread_term = term.clone();
    std::thread::spawn(move || {
        for line in stdout.lines() {
            if let Ok(line) = line {
                thread_term.write_line(&line).unwrap_or(());
            }
        }
    });

    // Process stderr in the main thread
    for line in stderr.lines() {
        if let Ok(line) = line {
            term.write_line(&format!("\x1b[31m{}\x1b[0m", line))
                .unwrap_or(());
        }
    }

    let status = child.wait()?;

    if !status.success() {
        anyhow::bail!(
            "Command failed with exit code {}: {} {}",
            status.code().unwrap_or(-1),
            program,
            args_str
        );
    }

    Ok(())
}

pub fn is_command_available(command: &str) -> bool {
    which::which(command).is_ok()
}
