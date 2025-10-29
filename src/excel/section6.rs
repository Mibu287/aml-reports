use std::io::{Read, Seek};

use crate::payload::section6::Section6;

impl Section6 {
    pub fn from_excel<RS>(_workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(Default::default())
    }
}
