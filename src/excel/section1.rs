use std::io::{Read, Seek};

use crate::{
    payload::section1::{
        ReportPreparer, ReportingEntity, ResponsiblePerson, Section1, TransactionLocation,
    },
    template::cell_value_from_key,
};

impl Section1 {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(Self {
            reporting_entity: ReportingEntity::from_excel(workbook)?,
            responsible_person: ResponsiblePerson::from_excel(workbook)?,
            report_preparer: ReportPreparer::from_excel(workbook)?,
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

impl ResponsiblePerson {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(Self {
            full_name: cell_value_from_key(
                "Phần I: Thông tin về người chịu trách nhiệm về phòng, chống rửa tiền - Họ và tên",
                workbook,
            )?,
            work_phone: cell_value_from_key(
                "Phần I: Thông tin về người chịu trách nhiệm về phòng, chống rửa tiền - Điện thoại nơi làm việc",
                workbook,
            )?,
            mobile_phone: cell_value_from_key(
                "Phần I: Thông tin về người chịu trách nhiệm về phòng, chống rửa tiền - Điện thoại di động",
                workbook,
            )?,
            position: cell_value_from_key(
                "Phần I: Thông tin về người chịu trách nhiệm về phòng, chống rửa tiền - Chức vụ",
                workbook,
            )?,
        })
    }
}

impl ReportPreparer {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(Self {
            full_name: cell_value_from_key(
                "Phần I: Thông tin về người lập báo cáo - Họ và tên",
                workbook,
            )?,
            work_phone: cell_value_from_key(
                "Phần I: Thông tin về người lập báo cáo - Điện thoại nơi làm việc",
                workbook,
            )?,
            mobile_phone: cell_value_from_key(
                "Phần I: Thông tin về người lập báo cáo - Điện thoại di động",
                workbook,
            )?,
            department: cell_value_from_key(
                "Phần I: Thông tin về người lập báo cáo - Bộ phận công tác",
                workbook,
            )?,
        })
    }
}
