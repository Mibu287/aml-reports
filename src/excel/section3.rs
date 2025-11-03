use std::io::{Read, Seek};

use crate::payload::section3::Section3;

impl Section3 {
    pub fn from_excel<RS>(
        _workbook: &mut calamine::Xlsx<RS>,
        _file_path: &std::path::Path,
    ) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(Section3 {
            related_individuals: None,
            related_organizations: None,
            additional_info: None,
        })
    }
}
