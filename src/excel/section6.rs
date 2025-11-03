use std::{
    io::{Read, Seek},
    path::PathBuf,
};

use anyhow::Context;

use crate::{
    codes::document_type::DocumentType,
    payload::section6::{Attachment, Section6},
};

impl Section6 {
    pub fn from_excel<RS>(
        workbook: &mut calamine::Xlsx<RS>,
        file_path: &std::path::Path,
    ) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Self::_from_excel(workbook, file_path)
            .with_context(|| format!("Lỗi xử lý dữ liệu Phần VI - Tài liệu đính kèm"))
    }

    fn _from_excel<RS>(
        _workbook: &mut calamine::Xlsx<RS>,
        file_path: &std::path::Path,
    ) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        let file_name = file_path
            .file_stem()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_default();

        let attachment_folder = {
            let mut folder = PathBuf::new();
            folder.push("input");
            folder.push(file_name);
            folder
        };
        if !attachment_folder.exists() || !attachment_folder.is_dir() {
            return Err(anyhow::anyhow!(
                "Không tìm thấy folder chứa các file đính kèm {:#?}. Bổ sung thêm folder đính kèm và đặt tên folder trùng với tên file.",
                attachment_folder.as_path()
            ));
        }

        let file_list = std::fs::read_dir(&attachment_folder)
            .with_context(|| format!("Không thể mở folder {:#?}", &attachment_folder))?;

        let mut attachments = vec![];

        for file in file_list {
            let file =
                file.with_context(|| format!("Không thể mở folder {:#?}", &attachment_folder))?;

            let file_name = file
                .path()
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            if file_name.starts_with("~$") {
                continue;
            }

            let file_prefix = file_name
                .split("_")
                .next()
                .unwrap_or_default()
                .to_string()
                .to_uppercase()
                .to_document_type()
                .with_context(|| {
                    format!(
                        "File đính kèm {:#?} có tiền tố không hợp lệ",
                        file.path().as_path()
                    )
                })?;

            let file_desc = file_name
                .strip_prefix(format!("{}_", file_prefix).as_str())
                .unwrap_or_default()
                .to_string();

            let file_ext = file
                .path()
                .extension()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();

            let file_mime = mime_guess::from_path(file.path())
                .first_or_octet_stream()
                .to_string()
                .into();

            let file_content = std::fs::read(file.path())
                .with_context(|| format!("Không thể đọc file {}", file.path().to_string_lossy()))?;

            let page_count = match file_ext.to_lowercase().as_str() {
                "pdf" => {
                    let doc =
                        lopdf::Document::load_mem(file_content.as_slice()).with_context(|| {
                            format!("Không thể đọc được file PDF {:#?}", file.path())
                        })?;
                    doc.get_pages().iter().count() as i32
                }
                _ => 1,
            };

            let file_size = file_content.len() as i64;

            attachments.push(Attachment {
                str_id: None,
                status: "ACTIVE".to_string().into(),
                attachment_type: file_prefix.into(),
                page_count: page_count.into(),
                description: file_desc.into(),
                file_name: file
                    .path()
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
                    .into(),
                file_type: file_ext.into(),
                file_size: file_size.into(),
                file: Default::default(),
                file_mime: file_mime,
                file_content: file_content.into(),
            });
        }

        Ok(Section6 { attachments })
    }
}
