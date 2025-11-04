use chrono::{DateTime, Local};
use colored::Colorize;
use shadow_rs::shadow;

shadow!(build);

pub fn print_build_info() {
    println!(
        "{}",
        r#"
+--------------------------------------------------------------------------------------+
|                                                                                      |
|                                                                                      |
|   ██████╗ ██╗   ██╗ █████╗ ███╗   ██╗████████╗    ██████╗ ███████╗██████╗ ████████╗  |
|  ██╔═══██╗██║   ██║██╔══██╗████╗  ██║╚══██╔══╝    ██╔══██╗██╔════╝██╔══██╗╚══██╔══╝  |
|  ██║   ██║██║   ██║███████║██╔██╗ ██║   ██║       ██║  ██║█████╗  ██████╔╝   ██║     |
|  ██║▄▄ ██║██║   ██║██╔══██║██║╚██╗██║   ██║       ██║  ██║██╔══╝  ██╔═══╝    ██║     |
|  ╚██████╔╝╚██████╔╝██║  ██║██║ ╚████║   ██║       ██████╔╝███████╗██║        ██║     |
|   ╚══▀▀═╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═══╝   ╚═╝       ╚═════╝ ╚══════╝╚═╝        ╚═╝     |
|                                                                                      |
|                                                                                      |
+--------------------------------------------------------------------------------------+
"#
        .green()
    );

    println!(
        "{}:      {}",
        "Chương trình".cyan().bold(),
        "Báo cáo giao dịch nghi ngờ gửi NHNN"
    );
    println!(
        "{}:    {}",
        "Người xây dựng".cyan().bold(),
        "Phòng Mô hình và công cụ quản trị rủi ro - Vietcombank"
    );
    println!(
        "{}:         {}",
        "Phiên bản".cyan().bold(),
        build::PKG_VERSION
    );
    println!(
        "{}:             {}",
        "Mã số".cyan().bold(),
        build::SHORT_COMMIT
    );
    println!(
        "{}: {}",
        "Lần cập nhật cuối".cyan().bold(),
        build::COMMIT_DATE
            .parse::<DateTime<Local>>()
            .map(|t| t.format("%Y-%m-%d %H:%M:%S %z").to_string())
            .unwrap_or("UNKNOWN".to_string())
    );
}
