use std::io::{Read, Seek};

use crate::payload::section3::Section3;

impl Section3 {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(Default::default())
    }
}
