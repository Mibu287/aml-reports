use std::io::{Read, Seek};

use crate::{
    codes::document_type::DocumentType,
    excel::section2::{get_cell_value, read_table_from_sheet},
    payload::section6::{Attachment, Section6},
};

impl Section6 {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        let sheet_key = "Phần VI. Tài liệu đính kèm";

        let (rows, col_map, base_coord) = read_table_from_sheet(workbook, sheet_key)?;

        let attachments = rows
            .into_iter()
            .map(|curr_row| {
                let cell_value_func =
                    |col_name: &str| get_cell_value(col_name, &col_map, base_coord, &curr_row);

                let file_path: std::path::PathBuf =
                    cell_value_func("Tài liệu đính kèm (viết theo tên file đính kèm)")
                        .unwrap_or_default()
                        .into();

                let file_name = file_path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string());

                let file_ext = file_path
                    .extension()
                    .map(|s| s.to_string_lossy().to_string());

                let file_mime = mime_guess::from_path(&file_path)
                    .first_or_octet_stream()
                    .to_string()
                    .into();
                let file_content = std::fs::read(&file_path).ok();
                let file_size = file_content.as_ref().map(|content| content.len() as i64);

                Attachment {
                    str_id: None,
                    status: "ACTIVE".to_string().into(),
                    attachment_type: cell_value_func("Loại tài liệu").to_document_type().into(),
                    page_count: cell_value_func("Số trang")
                        .unwrap_or_default()
                        .parse::<i32>()
                        .ok(),
                    description: cell_value_func("Mô tả tài liệu").into(),
                    file_name: file_name,
                    file_type: file_ext,
                    file_size: file_size,
                    file: Default::default(),
                    file_mime: file_mime,
                    file_content: file_content,
                }
            })
            .collect::<Vec<_>>();

        Ok(Section6 { attachments })
    }
}
