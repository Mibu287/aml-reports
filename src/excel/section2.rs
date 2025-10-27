use std::{
    collections::HashMap,
    io::{Read, Seek},
};

use calamine::{DataType, Reader};

use crate::{
    payload::{
        entities::{
            Account, AddrSimple, Bank, BeneficialOwners, CodeDesc, EnterpriseCode, Identification,
            Individual, IndividualLink, License, Occupation, Organization, OrganizationLink,
        },
        section2::Section2,
    },
    template::{cell_value_from_key, table_config_from_key},
    utils::excel::col_name_to_index,
};

impl Section2 {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: std::io::Seek + std::io::Read,
    {
        Ok(Self {
            individuals: Individual::from_excel(workbook)?,
            organizations: Organization::from_excel(workbook)?,
            beneficial_owners: BeneficialOwners::from_excel(workbook)?,
            additional_info: Some(cell_value_from_key("Phần II: Thông tin bổ sung", workbook)?),
        })
    }
}

fn read_table_from_sheet<RS>(
    workbook: &mut calamine::Xlsx<RS>,
    sheet_key: &str,
) -> anyhow::Result<(Vec<Vec<String>>, HashMap<String, String>, (u32, u32))>
where
    RS: Seek + Read,
{
    let table_config = table_config_from_key(sheet_key)?;
    let range = workbook.worksheet_range(&table_config.sheet)?;
    let header_row_idx = table_config.header_row - range.start().unwrap_or_default().0 - 1;
    let end_row_idx = {
        let mut end_row_idx = 0;
        for (row_idx, row_content) in range.rows().enumerate() {
            if row_idx as u32 <= header_row_idx {
                end_row_idx = row_idx;
                continue;
            }

            if row_content.iter().all(|value| value.is_empty()) {
                break;
            }

            end_row_idx = row_idx;
        }
        end_row_idx
    };

    let rows: Vec<Vec<String>> = range
        .rows()
        .enumerate()
        .filter(|(row_idx, _)| {
            row_idx.clone() as u32 > header_row_idx && row_idx.clone() <= end_row_idx
        })
        .map(|(_, row_content)| {
            row_content
                .iter()
                .map(|v| v.as_string().unwrap_or_default())
                .collect::<Vec<String>>()
        })
        .collect();

    Ok((
        rows,
        table_config.columns,
        range.start().unwrap_or_default(),
    ))
}

impl Individual {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Option<Vec<Self>>>
    where
        RS: std::io::Seek + std::io::Read,
    {
        let (rows, col_map, base_coord) = read_table_from_sheet(workbook, "Phần II. KHCN")?;

        let mut row_idx = 0;
        let mut persons: Vec<Individual> = vec![];
        while row_idx < rows.len() {
            let curr_row = &rows[row_idx];

            let cell_value_func = |col_name: &str| -> Option<String> {
                let col_name = col_map
                    .get(col_name)
                    .expect(format!("{} column not found", col_name).as_str());

                let col_idx = col_name_to_index(col_name, base_coord.into())
                    .expect(format!("Invalid column name {}", col_name).as_str());

                let value = curr_row[col_idx as usize].trim();

                if value.is_empty() {
                    None
                } else {
                    Some(value.to_string())
                }
            };

            let cif_value = cell_value_func("CIF");

            if !cif_value.is_none() {
                persons.push(Individual {
                    id: None,
                    existing_customer: Some("1".to_string()),
                    full_name: cell_value_func("Tên khách hàng"),
                    date_of_birth: cell_value_func("Ngày tháng năm sinh"),
                    age: None,
                    gender: cell_value_func("Giới tính"),
                    nationality: cell_value_func("Quốc tịch"),
                    occupation: Some(Occupation {
                        occupation_code: None,
                        description: cell_value_func("Nghề nghiệp"),
                        content: cell_value_func("Nếu Nghề nghiệp Khác"),
                    }),
                    position: None,
                    permanent_address: Some(AddrSimple {
                        street_address: cell_value_func("Địa chỉ đăng ký thường trú (Số nhà)"),
                        city_province: cell_value_func("Địa chỉ đăng ký thường trú (Tỉnh/TP)"),
                        district: cell_value_func("Địa chỉ đăng ký thường trú (Phường/Xã)"),
                        country: cell_value_func("Địa chỉ đăng ký thường trú (Quốc gia)"),
                        phone: None,
                    }),
                    current_address: Some(AddrSimple {
                        street_address: cell_value_func("Nơi ở hiện tại (Số nhà)"),
                        city_province: cell_value_func("Nơi ở hiện tại (Tỉnh/TP)"),
                        district: cell_value_func("Nơi ở hiện tại (Phường/Xã)"),
                        country: cell_value_func("Nơi ở hiện tại (Quốc gia)"),
                        phone: None,
                    }),
                    identifications: Some(vec![Identification {
                        id_type: cell_value_func("Loại định danh"),
                        id_number: cell_value_func("CMND/CCCD/Hộ chiếu/Định danh cá nhân"),
                        issue_date: cell_value_func("Ngày cấp (dd/mm/yyyy)"),
                        issuing_authority: cell_value_func("Cơ quan cấp"),
                        expiry_date: cell_value_func("Ngày hết hạn (dd/mm/yyyy)"),
                        place_of_issue: cell_value_func("Nơi cấp"),
                    }]),
                    phone_number: cell_value_func("Số điện thoại"),
                    education_level: None,
                    email: cell_value_func("Email"),
                    accounts: Some(vec![]),
                });
            }

            let account = Account {
                account_number: cell_value_func("Số tài khoản"),
                bank: Some(Bank {
                    bank_name: cell_value_func("Tên Ngân hàng"),
                    bank_code: cell_value_func("Mã Ngân hàng"),
                }),
                currency_type: cell_value_func("Loại tiền"),
                account_type: cell_value_func("Loại TK"),
                open_date: cell_value_func("Ngày mở"),
                status: cell_value_func("Trạng thái"),
                authorized_persons: None,
            };

            persons
                .last_mut()
                .map(|p| p.accounts.as_mut().map(|accounts| accounts.push(account)));
            row_idx += 1;
        }

        Ok(Some(persons))
    }
}

impl Organization {
    fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Option<Vec<Self>>>
    where
        RS: std::io::Seek + std::io::Read,
    {
        let sheet_key = "Phần II. KHTC";
        let (rows, col_map, base_coord) = read_table_from_sheet(workbook, sheet_key)?;

        let mut row_idx = 0;
        let mut orgs: Vec<Organization> = vec![];

        while row_idx < rows.len() {
            let curr_row = &rows[row_idx];

            let cell_value_func = |col_name: &str| -> Option<String> {
                let col_name = col_map
                    .get(col_name)
                    .expect(format!("{} column not found", col_name).as_str());

                let col_idx = col_name_to_index(col_name, base_coord.into())
                    .expect(format!("Invalid column name {}", col_name).as_str());

                let value = curr_row[col_idx as usize].trim();

                if value.is_empty() {
                    None
                } else {
                    Some(value.to_string())
                }
            };

            let cif_value = cell_value_func("CIF");

            if !cif_value.is_none() {
                orgs.push(Organization {
                    id: cell_value_func("STT").map(|v| {
                        v.parse::<i64>()
                            .expect(format!("Column STT in sheet `{}` invalid", sheet_key).as_str())
                    }),
                    existing_customer: "1".to_string().into(),
                    name: cell_value_func("Tên khách hàng"),
                    foreign_name: None,
                    short_name: None,
                    organization_type: CodeDesc {
                        type_code: cell_value_func("Loại hình tổ chức"),
                        description: cell_value_func("Loại hình tổ chức nếu chọn Khác"),
                    }
                    .into(),
                    address: AddrSimple {
                        street_address: cell_value_func("Số nhà"),
                        district: cell_value_func("Phường/Xã"),
                        city_province: cell_value_func("Tỉnh/TP"),
                        country: cell_value_func("Quốc gia"),
                        phone: cell_value_func("Số điện thoại"),
                    }
                    .into(),
                    establishment_license: License {
                        license_number: cell_value_func("MS doanh nghiệp/MS thuế"),
                        issue_date: cell_value_func("Ngày thành lập (dd/mm/yyyy)"),
                        issue_place: cell_value_func("Quốc gia thành lập"),
                    }
                    .into(),
                    enterprise_code: EnterpriseCode {
                        code: cell_value_func("MS doanh nghiệp/MS thuế"),
                        issue_date: cell_value_func("Ngày thành lập (dd/mm/yyyy)"),
                        issue_place: cell_value_func("Quốc gia thành lập"),
                    }
                    .into(),
                    business_sector: cell_value_func("Ngành nghề kinh doanh chính"),
                    phone_number: cell_value_func("Số điện thoại"),
                    website: None,
                    accounts: vec![].into(),
                    representatives: None,
                });
            }

            let account = Account {
                account_number: cell_value_func("Số tài khoản"),
                bank: Some(Bank {
                    bank_name: cell_value_func("Tên Ngân hàng"),
                    bank_code: cell_value_func("Mã Ngân hàng"),
                }),
                currency_type: cell_value_func("Loại tiền"),
                account_type: cell_value_func("Loại TK"),
                open_date: cell_value_func("Ngày mở"),
                status: cell_value_func("Trạng thái"),
                authorized_persons: None,
            };

            orgs.last_mut()
                .map(|p| p.accounts.as_mut().map(|accounts| accounts.push(account)));
            row_idx += 1;
        }

        Ok(orgs.into())
    }
}

impl BeneficialOwners {
    fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Option<Self>>
    where
        RS: std::io::Seek + std::io::Read,
    {
        Ok(None)
    }
}
