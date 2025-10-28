use std::{
    collections::HashMap,
    io::{Read, Seek},
};

use anyhow::Ok;
use calamine::{DataType, Reader};

use crate::{
    payload::section4::{Analysis, Clause, LegalBasis, ReportType, Section4, SuspiciousIndicator},
    template::{
        cell_value_from_key, legal_basis_mapping_from_key, mapping_from_key, value_list_from_key,
    },
    utils::{datetime::ConvertDateFormat, excel::read_cell_value},
};

impl Section4 {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(Section4 {
            report_type: ReportType::from_excel(workbook)?.into(),
            transaction_info: None,
            analysis: Analysis::from_excel(workbook)?.into(),
            conclusions: None,
            detection_date: cell_value_from_key(
                "Phần IV: Ngày phát hiện giao dịch đáng ngờ",
                workbook,
            )
            .ok()
            .convert_date_vn_to_iso(),
        })
    }
}

impl ReportType {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        let sheet_name = cell_value_from_key("Phần IV: Thông tin về giao dịch đáng ngờ", workbook)?;
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

        let report_key = "Phần IV: Loại báo cáo giao dịch đáng ngờ";
        let reports = mapping_from_key(report_key)?
            .into_iter()
            .filter(|(k, _)| selection.get(k).copied().unwrap_or(false))
            .map(|(k, v)| Clause {
                code: k.into(),
                description: v.into(),
            })
            .collect::<Vec<_>>();

        let indicator_key = "Phần IV: Dấu hiệu đáng ngờ";
        let indicators = mapping_from_key(indicator_key)?
            .into_iter()
            .filter(|(k, _)| selection.get(k).copied().unwrap_or(false))
            .map(|(k, v)| SuspiciousIndicator {
                code: k.into(),
                description: v.into(),
                other_content: None,
            })
            .collect::<Vec<_>>();

        Ok(Self {
            clauses: reports.into(),
            suspicious_indicators: indicators.into(),
        })
    }
}

impl Analysis {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        let sheet_key = "Phần IV: Thông tin về giao dịch đáng ngờ";
        let sheet_name = cell_value_from_key(sheet_key, workbook)?;

        let detail_key = "Phần IV: Mô tả, phân tích chi tiết";
        let detail_analysis = value_list_from_key(detail_key)?
            .into_iter()
            .map(|cell_name| read_cell_value(workbook, &sheet_name, &cell_name).unwrap_or_default())
            .filter(|v| !v.is_empty())
            .fold(String::new(), |mut acc, s| {
                if !acc.is_empty() {
                    acc.push_str("\n");
                }
                acc.push_str(&s);
                acc
            });

        Ok(Analysis {
            detail: detail_analysis.into(),
            legal_bases: LegalBasis::from_excel(workbook)?.into(),
        })
    }
}

impl LegalBasis {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Vec<Self>>
    where
        RS: Seek + Read,
    {
        let sheet_key = "Phần IV: Thông tin về giao dịch đáng ngờ";
        let sheet_name = cell_value_from_key(sheet_key, workbook)?;

        let legal_basis_key = "Phần IV: Cơ sở hợp lý để nghi ngờ";

        let legal_basis = legal_basis_mapping_from_key(legal_basis_key)?
            .into_iter()
            .map(|(field, basis)| {
                let notice_number = basis
                    .document_number
                    .and_then(|cell_name| read_cell_value(workbook, &sheet_name, &cell_name).ok())
                    .map(|s| if s.is_empty() { None } else { Some(s) })
                    .flatten();

                let basis_text = basis
                    .basis
                    .and_then(|cell_name| read_cell_value(workbook, &sheet_name, &cell_name).ok())
                    .map(|s| if s.is_empty() { None } else { Some(s) })
                    .flatten();

                LegalBasis {
                    report_type: field.into(),
                    notice_number: notice_number,
                    basis: basis_text,
                }
            })
            .collect::<Vec<_>>();

        Ok(legal_basis)
    }
}
