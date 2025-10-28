use std::{
    collections::HashMap,
    io::{Read, Seek},
};

use calamine::{DataType, Reader};

use crate::{
    payload::section5::{ProcessedTask, Section5},
    template::cell_value_from_key,
};

impl Section5 {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        let sheet_name = cell_value_from_key("Phần V: Công việc xử lý", workbook)?;
        let checked_box = cell_value_from_key("Dấu tick", workbook)?;
        let range = workbook.worksheet_range(&sheet_name)?;

        let selection = range
            .rows()
            .into_iter()
            .map(|row| {
                let key = row
                    .get(0)
                    .map(|c| c.get_string().unwrap_or_default().trim().to_string())
                    .unwrap_or(Default::default());

                let value = row
                    .get(1)
                    .map(|c| c.get_string().unwrap_or_default().trim().to_string())
                    .map(|c| c == checked_box) // R is for checkedbox (font: Wingdings 2)
                    .unwrap_or_default();

                (key, value)
            })
            .filter(|(k, v)| !k.is_empty() && *v)
            .collect::<HashMap<_, _>>();

        let get_desc_fn = |key: &str| -> Option<String> {
            let desc_key = format!("{}_desc", key);

            range
                .rows()
                .into_iter()
                .map(|row| {
                    let key = row
                        .get(0)
                        .map(|c| c.get_string().unwrap_or_default().trim().to_string())
                        .unwrap_or_default();
                    let value = row
                        .get(2)
                        .map(|c| c.get_string().unwrap_or_default().trim().to_string())
                        .unwrap_or_default();
                    (key, value)
                })
                .filter(|(k, _)| k == &desc_key)
                .map(|(_, v)| v)
                .collect::<Vec<_>>()
                .get(0)
                .map(|s| s.clone())
        };

        let processed_tasks = vec![
            ("1", "Từ chối thực hiện giao dịch"),
            ("2", "Tạm khóa tài khoản"),
            ("3", "Chấm dứt thiết lập giao dịch với khách hàng"),
            ("4", "Giám sát sau giao dịch"),
            ("5", "Đưa vào hệ thống cảnh báo của đối tượng báo cáo"),
            ("6", "Ngân hàng đã có công văn gửi Cơ quan nhà nước có thẩm quyền"),
            ("7", "Ngân hàng nhận được công văn của Cơ quan nhà nước có thẩm quyền yêu cầu cung cấp thông tin, tài liệu"),
            ("8", "Tạm ngừng cung cấp dịch vụ ngân hàng điện tử"),
            ("0", "Công việc khác"),
        ]
        .into_iter()
        .filter(|(code, _)| selection.get(*code).cloned().unwrap_or_default())
        .map(|(code, description)| ProcessedTask {
            code: code.to_string().into(),
            description: description.to_string().into(),
            documents: Some(vec![]),
            other_content: get_desc_fn(code),
        }).collect::<Vec<_>>();

        Ok(Section5 {
            processed_tasks: processed_tasks.into(),
        })
    }
}
