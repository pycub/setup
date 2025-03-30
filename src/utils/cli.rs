use anyhow::Result;
use colored::*;
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::io::{self, Write};

/// Prints a section header with consistent styling
pub fn print_section_header(title: &str) {
    println!("\n{}", title.green().bold());
    println!("{}", "=".repeat(title.len()).green());
}

/// Prints a success message
pub fn print_success(message: &str) {
    println!("{} {}", "✓".green(), message);
}

/// Prints an error message
pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red(), message);
}

/// Prints a warning message
pub fn print_warning(message: &str) {
    println!("{} {}", "!".yellow(), message);
}

/// Prints an info message
pub fn print_info(message: &str) {
    println!("{} {}", "i".blue(), message);
}

/// Ask for user confirmation with a customizable default
pub fn confirm(prompt: &str, default: bool) -> Result<bool> {
    Ok(Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default)
        .interact()?)
}

/// Ask for user input with a customizable default
pub fn input<T>(prompt: &str, default: Option<T>) -> Result<T>
where
    T: std::str::FromStr + std::fmt::Display,
    T::Err: std::fmt::Debug,
{
    let mut input = Input::with_theme(&ColorfulTheme::default());
    input.with_prompt(prompt);

    if let Some(default_value) = default {
        input.default(default_value);
    }

    Ok(input.interact_text()?)
}

/// Ask user to select from a list of options
pub fn select<T: AsRef<str>>(prompt: &str, items: &[T]) -> Result<usize> {
    Ok(Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items)
        .default(0)
        .interact()?)
}

/// Ask user to select multiple options from a list
pub fn multiselect<T: AsRef<str>>(
    prompt: &str,
    items: &[T],
    defaults: &[bool],
) -> Result<Vec<usize>> {
    Ok(
        dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(items)
            .defaults(defaults)
            .interact()?,
    )
}

/// Clear the terminal screen
pub fn clear_screen() -> Result<()> {
    // ANSI escape code to clear screen and move cursor to home position
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush()?;
    Ok(())
}

/// Display a spinner with message during an operation
pub fn with_spinner<F, T>(message: &str, operation: F) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let result = operation();

    spinner.finish_and_clear();
    result
}
