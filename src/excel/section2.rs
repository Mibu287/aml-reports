use std::{
    collections::HashMap,
    io::{Read, Seek},
};

use calamine::{DataType, Reader};

use crate::{
    codes::{
        account_status::BankAccountStatusCode,
        account_type::BankAccountTypeCode, country::CountryCode,
        gender::GenderCode, occupation::OccupationCode,
        personal_id::PersonalIdCode,
    },
    payload::{
        entities::{
            Account, AddrSimple, Bank, BeneficialOwners, CodeDesc, EnterpriseCode, Identification,
            Individual, License, Occupation, Organization, Representative,
        },
        section2::Section2,
    },
    template::{cell_value_from_key, table_config_from_key},
    utils::{datetime::ConvertDateFormat, excel::col_name_to_index},
};

impl Section2 {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
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

fn get_cell_value(
    col_name: &str,
    col_map: &HashMap<String, String>,
    base_coord: (u32, u32),
    curr_row: &Vec<String>,
) -> Option<String> {
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
}

impl Individual {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Option<Vec<Self>>>
    where
        RS: Read + Seek,
    {
        let accounts = Account::from_excel(workbook)?;
        let (rows, col_map, base_coord) = read_table_from_sheet(workbook, "Phần II. KHCN")?;

        let persons = rows
            .into_iter()
            .map(|curr_row| {
                let cell_value_func =
                    |col_name: &str| get_cell_value(col_name, &col_map, base_coord, &curr_row);

                let cif_value = cell_value_func("CIF").unwrap_or_default();

                Individual {
                    id: cif_value.parse::<i64>().ok(),
                    existing_customer: if cif_value.is_empty() {
                        None
                    } else {
                        "1".to_string().into()
                    },
                    full_name: cell_value_func("Tên khách hàng"),
                    date_of_birth: cell_value_func("Ngày tháng năm sinh (dd/mm/yyyy)")
                        .convert_date_vn_to_iso(),
                    age: None,
                    gender: cell_value_func("Giới tính").map(|v| v.to_gender_code_owned()),
                    nationality: cell_value_func("Quốc tịch"),
                    occupation: Occupation {
                        occupation_code: cell_value_func("Nghề nghiệp")
                            .map(|v| v.to_occupation_code_owned()),
                        description: cell_value_func("Nghề nghiệp"),
                        content: cell_value_func("Nếu Nghề nghiệp Khác"),
                    }
                    .into(),
                    position: None,
                    permanent_address: Some(AddrSimple {
                        street_address: cell_value_func("Địa chỉ đăng ký thường trú (Số nhà)"),
                        city_province: cell_value_func("Địa chỉ đăng ký thường trú (Tỉnh/TP)"),
                        district: cell_value_func("Địa chỉ đăng ký thường trú (Phường/Xã)"),
                        country: cell_value_func("Địa chỉ đăng ký thường trú (Quốc gia)")
                            .map(|v| v.to_country_code_owned()),
                        phone: None,
                    }),
                    current_address: Some(AddrSimple {
                        street_address: cell_value_func("Nơi ở hiện tại (Số nhà)"),
                        city_province: cell_value_func("Nơi ở hiện tại (Tỉnh/TP)"),
                        district: cell_value_func("Nơi ở hiện tại (Phường/Xã)"),
                        country: cell_value_func("Nơi ở hiện tại (Quốc gia)")
                            .map(|v| v.to_country_code_owned()),
                        phone: None,
                    }),
                    identifications: Some(vec![Identification {
                        id_type: cell_value_func("Loại định danh")
                            .map(|v| v.to_personal_id_code_owned()),
                        id_number: cell_value_func("CMND/CCCD/Hộ chiếu/Định danh cá nhân"),
                        issue_date: cell_value_func("Ngày cấp (dd/mm/yyyy)")
                            .convert_date_vn_to_iso(),
                        issuing_authority: cell_value_func("Cơ quan cấp"),
                        expiry_date: cell_value_func("Ngày hết hạn (dd/mm/yyyy)")
                            .convert_date_vn_to_iso(),
                        place_of_issue: cell_value_func("Nơi cấp"),
                    }]),
                    phone_number: cell_value_func("Số điện thoại"),
                    education_level: None,
                    email: cell_value_func("Email"),
                    accounts: accounts.get(&cif_value).cloned(),
                }
            })
            .collect::<Vec<_>>();

        Ok(persons.into())
    }
}

impl Organization {
    fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Option<Vec<Self>>>
    where
        RS: Read + Seek,
    {
        let accounts = Account::from_excel(workbook)?;
        let rep_persons = Representative::from_excel(workbook)?;

        let sheet_key = "Phần II. KHTC";
        let (rows, col_map, base_coord) = read_table_from_sheet(workbook, sheet_key)?;

        let orgs = rows
            .into_iter()
            .map(|curr_row| {
                let cell_value_func =
                    |col_name: &str| get_cell_value(col_name, &col_map, base_coord, &curr_row);

                let cif_value = cell_value_func("CIF").unwrap_or_default();

                Organization {
                    id: cif_value.parse::<i64>().ok(),
                    existing_customer: if cif_value.is_empty() {
                        None
                    } else {
                        "1".to_string().into()
                    },
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
                        country: cell_value_func("Quốc gia").map(|v| v.to_country_code_owned()),
                        phone: cell_value_func("Số điện thoại"),
                    }
                    .into(),
                    establishment_license: License {
                        license_number: cell_value_func("Giấy phép thành lập số"),
                        issue_date: cell_value_func("Ngày cấp giấy phép (dd/mm/yyyy)")
                            .convert_date_vn_to_iso(),
                        issue_place: cell_value_func("Nơi cấp giấy phép"),
                    }
                    .into(),
                    enterprise_code: EnterpriseCode {
                        code: cell_value_func("MS doanh nghiệp/MS thuế"),
                        issue_date: cell_value_func("Ngày cấp MST (dd/mm/yyyy)")
                            .convert_date_vn_to_iso(),
                        issue_place: cell_value_func("Quốc gia cấp MST"),
                    }
                    .into(),
                    business_sector: cell_value_func("Ngành nghề kinh doanh chính"),
                    phone_number: cell_value_func("Số điện thoại"),
                    website: cell_value_func("Địa chỉ trang thông tin điện tử của doanh nghiệp"),
                    accounts: accounts.get(&cif_value).cloned().into(),
                    representatives: rep_persons.get(&cif_value).cloned().into(),
                }
            })
            .collect::<Vec<_>>();

        Ok(orgs.into())
    }
}

impl Account {
    fn from_excel<RS>(
        workbook: &mut calamine::Xlsx<RS>,
    ) -> anyhow::Result<HashMap<String, Vec<Self>>>
    where
        RS: Read + Seek,
    {
        let sheet_key = "Phần II. Tài khoản";
        let (rows, col_map, base_coord) = read_table_from_sheet(workbook, sheet_key)?;

        let accounts = rows
            .into_iter()
            .map(|curr_row| {
                let cell_value_func =
                    |col_name: &str| get_cell_value(col_name, &col_map, base_coord, &curr_row);

                let cif_value = cell_value_func("CIF").unwrap_or_default();

                let account = Account {
                    account_number: cell_value_func("Số tài khoản"),
                    bank: Some(Bank {
                        bank_name: cell_value_func("Tên Ngân hàng"),
                        bank_code: cell_value_func("Mã Ngân hàng")
                            .map(|v| v.split("-").next().unwrap_or_default().trim().to_string()),
                    }),
                    currency_type: cell_value_func("Loại tiền")
                        .map(|v| v.split("-").next().unwrap_or_default().trim().to_string()),
                    account_type: cell_value_func("Loại TK")
                        .map(|v| v.to_bank_account_type_code_owned()),
                    open_date: cell_value_func("Ngày mở").convert_date_vn_to_iso(),
                    status: cell_value_func("Trạng thái")
                        .map(|v| v.to_bank_account_status_code_owned()),
                    authorized_persons: None,
                };

                (cif_value, account)
            })
            .fold(
                HashMap::<String, Vec<Account>>::new(),
                |mut acc, (cif, account)| {
                    acc.entry(cif).or_default().push(account);
                    acc
                },
            );

        Ok(accounts)
    }
}

impl Representative {
    fn from_excel<RS>(
        workbook: &mut calamine::Xlsx<RS>,
    ) -> anyhow::Result<HashMap<String, Vec<Self>>>
    where
        RS: Read + Seek,
    {
        let sheet_key = "Phần II. Người đại diện";
        let (rows, col_map, base_coord) = read_table_from_sheet(workbook, sheet_key)?;

        let representatives = rows
            .into_iter()
            .map(|curr_row| {
                let cell_value_func =
                    |col_name: &str| get_cell_value(col_name, &col_map, base_coord, &curr_row);

                let cif_value = cell_value_func("CIF").unwrap_or_default();

                let rep = Representative {
                    id: cif_value.parse::<i64>().ok(),
                    full_name: cell_value_func("Họ và tên"),
                    date_of_birth: cell_value_func("Ngày sinh").convert_date_vn_to_iso(),
                    occupation: Occupation {
                        occupation_code: cell_value_func("Nghề nghiệp")
                            .map(|v| v.to_occupation_code_owned()),
                        description: cell_value_func("Nghề nghiệp"),
                        content: cell_value_func("Nếu Nghề nghiệp Khác"),
                    }
                    .into(),
                    position: cell_value_func("Chức vụ/vị trí việc làm"),
                    permanent_address: AddrSimple {
                        street_address: cell_value_func("Địa chỉ đăng ký thường trú (Số nhà)"),
                        city_province: cell_value_func("Địa chỉ đăng ký thường trú (Tỉnh/TP)"),
                        district: cell_value_func("Địa chỉ đăng ký thường trú (Phường/Xã)"),
                        country: cell_value_func("Địa chỉ đăng ký thường trú (Quốc gia)")
                            .map(|v| v.to_country_code_owned()),
                        phone: None,
                    }
                    .into(),
                    current_address: AddrSimple {
                        street_address: cell_value_func("Nơi ở hiện tại (Số nhà)"),
                        city_province: cell_value_func("Nơi ở hiện tại (Tỉnh/TP)"),
                        district: cell_value_func("Nơi ở hiện tại (Phường/Xã)"),
                        country: cell_value_func("Nơi ở hiện tại (Quốc gia)")
                            .map(|v| v.to_country_code_owned()),
                        phone: None,
                    }
                    .into(),
                    phone_number: cell_value_func("Điện thoại liên lạc"),
                    nationality: cell_value_func("Quốc tịch").map(|v| v.to_country_code_owned()),
                    identifications: Some(vec![Identification {
                        id_type: cell_value_func("Loại định danh")
                            .map(|v| v.to_personal_id_code_owned()),
                        id_number: cell_value_func("CMND/CCCD/Hộ chiếu/Định danh cá nhân"),
                        issue_date: cell_value_func("Ngày cấp (dd/mm/yyyy)")
                            .convert_date_vn_to_iso(),
                        issuing_authority: cell_value_func("Cơ quan cấp"),
                        expiry_date: None,
                        place_of_issue: cell_value_func("Nơi cấp"),
                    }]),
                };

                (cif_value, rep)
            })
            .fold(
                HashMap::<String, Vec<Representative>>::new(),
                |mut acc, (cif, rep)| {
                    acc.entry(cif).or_default().push(rep);
                    acc
                },
            );

        Ok(representatives)
    }
}

impl BeneficialOwners {
    fn from_excel<RS>(_workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Option<Self>>
    where
        RS: Seek + Read,
    {
        Ok(None)
    }
}
