use shadow_rs::shadow;

shadow!(build);

pub fn print_build_info() {
    println!("Chương trình:      {}", "Báo cáo giao dịch nghi ngờ gửi NHNN");
    println!("Người xây dựng:    {}", "Phòng Mô hình và công cụ quản trị rủi ro");
    println!("Phiên bản:         {}", build::PKG_VERSION);
    println!("Mã số:             {}", build::COMMIT_DATE);
    println!("Lần cập nhật cuối: {}", build::COMMIT_DATE);
}
