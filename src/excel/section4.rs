use std::{
    collections::HashMap,
    io::{Read, Seek},
};

use calamine::{DataType, Reader};

use crate::{
    payload::section4::{Clause, ReportType, Section4, SuspiciousIndicator},
    template::{cell_value_from_key, mapping_from_key},
};

impl Section4 {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(Section4 {
            report_type: ReportType::from_excel(workbook)?.into(),
            transaction_info: None,
            analysis: None,
            conclusions: None,
            detection_date: None,
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
