use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

pub fn success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

pub fn error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message);
}

pub fn info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}

pub fn warning(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message);
}

pub fn create_spinner(message: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));
    spinner
}

pub fn create_progress(total: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len}")
            .unwrap()
            .progress_chars("█▓▒░ "),
    );
    pb.set_message(message.to_string());
    pb
}

