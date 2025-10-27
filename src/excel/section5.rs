use std::io::{Read, Seek};

use crate::payload::section5::Section5;

impl Section5 {
    pub fn from_excel<RS>(_workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(Default::default())
    }
}
