use std::io::{Read, Seek};

use crate::{
    payload::section1::{ReportingEntity, Section1, TransactionLocation},
    template::cell_value_from_key,
};

impl Section1 {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(Self {
            reporting_entity: ReportingEntity::from_excel(workbook)?,
            responsible_person: Default::default(),
            report_preparer: Default::default(),
        })
    }
}

impl ReportingEntity {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(Self {
            name: None,
            code: None,
            address: None,
            transaction_location: TransactionLocation {
            transaction_point_name: cell_value_from_key(
                "Phần I: Tên điểm phát sinh giao dịch hoặc đơn vị quản lý tài khoản",
                workbook,
            )?,
            street_address: cell_value_from_key(
                "Phần I: Địa chỉ điểm phát sinh giao dịch hoặc địa chỉ đơn vị quản lý tài khoản",
                workbook,
            )?,
            district: cell_value_from_key("Phần I: Phường/Xã", workbook)?,
            city_province: cell_value_from_key("Phần I: Tỉnh/Thành phố", workbook)?,
            country: cell_value_from_key("Phần I: Quốc gia", workbook)?,
            phone: cell_value_from_key("Phần I: Điện thoại", workbook)?,
            },
            email: cell_value_from_key("Phần I: Địa chỉ email của đơn vị", workbook)?,
        })
    }
}
