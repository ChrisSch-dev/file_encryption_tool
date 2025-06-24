use indicatif::{ProgressBar, ProgressStyle};

pub fn show_progress_bar(length: u64, msg: &str) -> ProgressBar {
    let pb = ProgressBar::new(length);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));
    pb.set_message(msg);
    pb
}