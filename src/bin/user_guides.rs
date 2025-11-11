use std::io::{BufRead, Write};

use aml::{
    build::print_build_info, codes::document_type::DOCUMENT_TYPES, template::value_list_from_key,
};
use colored::Colorize;
use crossterm::{cursor, queue, terminal};
use tabled::Tabled;

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

fn spacer(n_rows: usize) {
    // Spacer
    for _ in 0..n_rows {
        println!("");
    }
}

fn header() {
    spacer(3);

    println!(
        "{}",
        "Các bước thực hiện gửi báo cáo giao dịch đáng ngờ lên website NHNN"
            .green()
            .bold()
    );
}

fn step_1() {
    spacer(1);

    println!(
        "{:<10}: {}",
        "Bước 1".green().bold(),
        "Chuẩn bị biểu mẫu báo cáo và các file đính kèm."
    );
}

fn step_1_1() {
    println!(
        "{:<10}: {}{}{}{}",
        "Bước 1.1".green().bold(),
        "Lưu file Excel biểu mẫu báo cáo vào folder con ",
        "'input'".on_green(),
        ". Ví dụ: lưu file ",
        "'input/example_aml_report.xlsx'".on_green(),
    );
}

fn step_1_2() {
    println!(
        "{:<10}: {}{}{}{}{}",
        "Bước 1.2".green().bold(),
        "Tạo folder con trong ",
        "'input'".on_green(),
        " để lưu các file đính kèm. ",
        "Tên của folder mới trùng tên với biểu mẫu. Ví dụ: tạo mới folder ",
        "'input/example_aml_report'".on_green(),
    );
}

fn step_1_3() {
    println!(
        "{:<10}: {}",
        "Bước 1.3".green(),
        "Chuẩn bị các file đính kèm theo danh sách như sau:"
    );
    #[derive(Tabled)]
    struct Attachment {
        #[tabled(rename = "STT", order = 0)]
        sequence: usize,
        #[tabled(rename = "Loại tài liệu", order = 1)]
        doc_name: String,
        #[tabled(rename = "Tiền tố", order = 2)]
        prefix: String,
        #[tabled(rename = "Ví dụ", order = 3)]
        example: String,
        #[tabled(rename = "Ghi chú", order = 4)]
        required: String,
    }

    let required_files =
        value_list_from_key("Phần VI. Tài liệu đính kèm - Tài liệu bắt buộc").unwrap_or_default();
    let attachments = DOCUMENT_TYPES
        .into_iter()
        .enumerate()
        .map(|(index, (prefix, doc_name))| Attachment {
            sequence: index + 1,
            doc_name: doc_name.to_string(),
            prefix: prefix.to_string(),
            example: format!("{}_{}.xlsx", prefix, doc_name),
            required: {
                let match_count = required_files
                    .iter()
                    .filter(|required_doc| required_doc.as_str() == doc_name)
                    .count();

                if match_count > 0 {
                    "Bắt buộc".on_green()
                } else {
                    "".normal()
                }
                .to_string()
            },
        })
        .collect::<Vec<_>>();

    let tbl = {
        use tabled::settings::*;

        let mut tbl = tabled::Table::new(attachments);
        tbl.with(Style::modern_rounded());

        let header = object::Rows::first();
        let colored_header = Modify::new(header).with(Color::FG_BRIGHT_GREEN);
        tbl.with(colored_header);
        tbl
    };
    println!("{}", tbl);
}

fn step_2() {
    spacer(1);

    println!(
        "{:<10}: {}{}{}",
        "Bước 2".green().bold(),
        "Bấm vào file ",
        "'validate-reports'".on_green(),
        " để thực hiện kiểm tra và sửa lỗi biểu mẫu báo cáo và các tài liệu đính kèm."
    );
}

fn step_3() {
    spacer(1);

    println!(
        "{:<10}: {}{}{}{}",
        "Bước 3".green().bold(),
        "Bấm vào file ",
        "'send-aml-reports'".on_green(),
        " để thực hiện gửi báo cáo lên website của NHNN. ",
        "Ứng dụng sẽ mở trình duyệt Chrome và yêu cầu người dùng nhập username, pasword."
    );
}

fn main() {
    print_build_info();

    let steps = [header, step_1, step_1_1, step_1_2, step_1_3, step_2, step_3];

    for step in steps.into_iter() {
        step();
        wait_for_user();
    }
}
