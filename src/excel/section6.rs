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

                Attachment {
                    str_id: None,
                    status: "ACTIVE".to_string().into(),
                    attachment_type: cell_value_func("Loại tài liệu").to_document_type().into(),
                    page_count: cell_value_func("Số trang")
                        .unwrap_or_default()
                        .parse::<i32>()
                        .ok()
                        .into(),
                    description: cell_value_func("Mô tả tài liệu").into(),
                    file_name: cell_value_func("Tài liệu đính kèm (viết theo tên file đính kèm)")
                        .into(),
                    file_type: None,
                    file_size: None,
                    file: None,
                }
            })
            .collect::<Vec<_>>();

        Ok(Section6 { attachments })
    }
}
