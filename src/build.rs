use chrono::{DateTime, Local};
use colored::Colorize;
use shadow_rs::shadow;

shadow!(build);

pub fn print_build_info() {
    const BANNER: &'static str = r#"
+=================================================================================================+
|                                                                                                 |
|                                                                                                 |
|   ██╗   ██╗██╗███████╗████████╗ ██████╗ ██████╗ ███╗   ███╗██████╗  █████╗ ███╗   ██╗██╗  ██╗   |
|   ██║   ██║██║██╔════╝╚══██╔══╝██╔════╝██╔═══██╗████╗ ████║██╔══██╗██╔══██╗████╗  ██║██║ ██╔╝   |
|   ██║   ██║██║█████╗     ██║   ██║     ██║   ██║██╔████╔██║██████╔╝███████║██╔██╗ ██║█████╔╝    |
|   ╚██╗ ██╔╝██║██╔══╝     ██║   ██║     ██║   ██║██║╚██╔╝██║██╔══██╗██╔══██║██║╚██╗██║██╔═██╗    |
|    ╚████╔╝ ██║███████╗   ██║   ╚██████╗╚██████╔╝██║ ╚═╝ ██║██████╔╝██║  ██║██║ ╚████║██║  ██╗   |
|     ╚═══╝  ╚═╝╚══════╝   ╚═╝    ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝   |
|                                                                                                 |
|                                                                                                 |
+=================================================================================================+
    "#;
    println!("{}", BANNER.green());

    println!(
        "{}:      {}",
        "Chương trình".green().bold(),
        "Báo cáo giao dịch nghi ngờ gửi NHNN"
    );
    println!(
        "{}:    {}",
        "Người xây dựng".green().bold(),
        "Phòng Mô hình và công cụ quản trị rủi ro"
    );
    println!(
        "{}:     {}",
        "Người sử dụng".green().bold(),
        "Phòng Phòng chống rửa tiền"
    );
    println!(
        "{}:         {}",
        "Phiên bản".green().bold(),
        build::PKG_VERSION
    );
    println!(
        "{}:             {}",
        "Mã số".green().bold(),
        build::SHORT_COMMIT
    );
    println!(
        "{}: {}",
        "Lần cập nhật cuối".green().bold(),
        build::COMMIT_DATE
            .parse::<DateTime<Local>>()
            .map(|t| t.format("%Y-%m-%d %H:%M:%S %z").to_string())
            .unwrap_or("UNKNOWN".to_string())
    );
}
