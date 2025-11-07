use std::{
    collections::HashMap,
    io::{Read, Seek},
};

use anyhow::Context;
use calamine::{DataType, Reader};

use crate::{
    payload::section5::{Document, ProcessedTask, Section5},
    template::cell_value_from_key,
    utils::datetime::ConvertDateFormat,
};

impl Section5 {
    pub fn from_excel<RS>(
        workbook: &mut calamine::Xlsx<RS>,
        _file_path: &std::path::Path,
    ) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Self::_from_excel(workbook, _file_path)
            .with_context(|| format!("Lỗi xử lý dữ liệu Phần V: Công việc xử lý"))
    }

    fn _from_excel<RS>(
        workbook: &mut calamine::Xlsx<RS>,
        _file_path: &std::path::Path,
    ) -> anyhow::Result<Self>
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
                    .map(|c| c == checked_box)
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

        #[derive(Copy, Clone)]
        enum DocType {
            In,
            Out,
        }

        let get_doc_fn = |key: &str, doc_type: DocType| -> anyhow::Result<Option<Document>> {
            let doc_key = match doc_type {
                DocType::In => format!("{}_in_doc", key),
                DocType::Out => format!("{}_out_doc", key),
            };

            let maybe_document = range
                .rows()
                .into_iter()
                .map(|row| {
                    let key = row
                        .get(0)
                        .map(|c| c.get_string().unwrap_or_default().trim().to_string())
                        .unwrap_or_default();
                    let doc_number = row
                        .get(3)
                        .map(|c| c.get_string().unwrap_or_default().trim().to_string());
                    let doc_date = row
                        .get(5)
                        .map(|d| d.get_string().unwrap_or_default().trim().to_string());
                    let unit = row
                        .get(7)
                        .map(|d| d.get_string().unwrap_or_default().trim().to_string());
                    (key, (doc_number, doc_date, unit))
                })
                .filter(|(k, _)| k == &doc_key)
                .map(
                    |(_, (doc_number, doc_date, unit))| -> anyhow::Result<Document> {
                        let doc_type = match (key, doc_type) {
                            ("6", DocType::In) => Some("0"),
                            ("7", DocType::In) => Some("1"),
                            ("7", DocType::Out) => Some("2"),
                            _ => None,
                        }
                        .map(|s| s.to_string());

                        let context_fn = || {
                            format!(
                                "Ngày '{}' của công văn '{}' không hợp lý.",
                                doc_date.clone().unwrap_or_default(),
                                doc_number.clone().unwrap_or_default(),
                            )
                        };

                        let doc_date =
                            doc_date.convert_date_vn_to_iso().with_context(context_fn)?;

                        Ok(Document {
                            doc_type: doc_type,
                            doc_number: doc_number,
                            doc_date: doc_date,
                            unit: unit,
                        })
                    },
                )
                .next();

            match maybe_document {
                None => Ok(None),
                Some(Err(err)) => Err(err),
                Some(Ok(document)) => Ok(Some(document)),
            }
        };

        let processed_tasks = vec![
            ("1", "Từ chối thực hiện giao dịch"),
            ("2", "Tạm khóa tài khoản"),
            ("3", "Chấm dứt thiết lập giao dịch với khách hàng"),
            ("4", "Giám sát sau giao dịch"),
            ("5", "Đưa vào hệ thống cảnh báo của đối tượng báo cáo"),
            (
                "6",
                "Ngân hàng đã có công văn gửi Cơ quan nhà nước có thẩm quyền",
            ),
            (
                "7",
                "Ngân hàng nhận được công văn của Cơ quan nhà nước có thẩm quyền yêu cầu cung cấp thông tin, tài liệu",
            ),
            ("8", "Tạm ngừng cung cấp dịch vụ ngân hàng điện tử"),
            ("0", "Công việc khác"),
        ];

        let processed_tasks = processed_tasks
            .into_iter()
            .filter(|(code, _)| selection.get(*code).cloned().unwrap_or_default())
            .map(
                |(code, description)| -> (anyhow::Result<ProcessedTask>, &str) {
                    let in_doc = get_doc_fn(code, DocType::In);
                    let out_doc = get_doc_fn(code, DocType::Out);

                    let (in_doc, out_doc) = match (in_doc, out_doc) {
                        (Err(err), _) => return (Err(err), description),
                        (_, Err(err)) => return (Err(err), description),
                        (Ok(in_doc), Ok(out_doc)) => (in_doc, out_doc),
                    };

                    let documents = match (in_doc, out_doc) {
                        (None, None) => None,
                        (None, Some(out_doc)) => Some(vec![out_doc]),
                        (Some(in_doc), None) => Some(vec![in_doc]),
                        (Some(in_doc), Some(out_doc)) => Some(vec![in_doc, out_doc]),
                    };

                    let task = Ok(ProcessedTask {
                        code: code.to_string().into(),
                        description: description.to_string().into(),
                        documents: documents,
                        other_content: get_desc_fn(code),
                    });
                    (task, description)
                },
            )
            .fold(
                anyhow::Result::<Vec<ProcessedTask>>::Ok(Default::default()),
                |final_result, (task, description)| {
                    let mut final_result = final_result?;
                    let task = task.with_context(|| {
                        format!(
                            "Lỗi xử lý dữ liệu cho công việc đã thực hiện '{}'",
                            description
                        )
                    })?;
                    final_result.push(task);
                    Ok(final_result)
                },
            )?;

        Ok(Section5 {
            processed_tasks: processed_tasks.into(),
        })
    }
}
