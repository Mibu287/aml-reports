use std::io::{Read, Seek};

use crate::payload::section4::Section4;

impl Section4 {
    pub fn from_excel<RS>(_workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(Default::default())
    }
}
