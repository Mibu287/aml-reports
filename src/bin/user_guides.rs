use std::io::{BufRead, Write};

use aml::build::print_build_info;
use colored::Colorize;
use crossterm::{cursor, queue, terminal};

fn wait_for_user() {
    // Print message
    let message = "Bấm Enter để tiếp tục";
    println!("{}", message.bold());

    // Move cursor up and blink
    {
        let mut stdout = std::io::stdout();
        let message_length = message.chars().count() as u16;
        queue!(
            stdout,
            cursor::MoveUp(1),
            cursor::MoveRight(message_length + 1),
            cursor::EnableBlinking
        )
        .unwrap();
        stdout.flush().unwrap_or_default();
    }

    // Wait for user
    let stdin = std::io::stdin();
    let _ = stdin.lock().lines().next();

    // Clear wait message
    {
        let mut stdout = std::io::stdout();
        queue!(
            stdout,
            cursor::MoveUp(1),
            terminal::Clear(terminal::ClearType::CurrentLine),
        )
        .unwrap_or_default();
        stdout.flush().unwrap_or_default();
    }
}

fn main() {
    // Banner
    print_build_info();

    // Spacer
    for _ in 0..3 {
        println!("");
    }

    // Header
    println!(
        "{}",
        "Các bước thực hiện gửi báo cáo giao dịch đáng ngờ lên website NHNN"
            .green()
            .bold()
    );
    println!("");

    wait_for_user();

    // Steps
    println!(
        "{:<10}: {}",
        "Bước 1".green().bold(),
        "Chuẩn bị biểu mẫu báo cáo và các file đính kèm."
    );
    wait_for_user();

    println!(
        "{:<10}: {}{}{}{}",
        "Bước 1.1".green().bold(),
        "Lưu file Excel biểu mẫu báo cáo vào folder con ",
        "'input'".on_green(),
        ". Ví dụ: lưu file ",
        "'input/example_aml_report.xlsx'".on_green(),
    );
    wait_for_user();

    println!(
        "{:<10}: {}{}{}{}{}",
        "Bước 1.2".green().bold(),
        "Tạo folder con trong ",
        "'input'".on_green(),
        " để lưu các file đính kèm. ",
        "Tên của folder mới trùng tên với biểu mẫu. Ví dụ: tạo mới folder ",
        "'input/example_aml_report'".on_green(),
    );
    wait_for_user();

    println!("");
}
