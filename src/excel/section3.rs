use std::{
    collections::HashMap,
    io::{Read, Seek},
};

use anyhow::Context;

use crate::{
    codes::{
        account_status::AccountStatusCode, account_type::AccountTypeCode, age_range::AgeRangeCode,
        country::CountryCode, currency::CurrencyCode, gender::GenderCode,
        occupation::OccupationCode, personal_id::PersonalIdCode,
    },
    excel::{get_cell_value, read_table_from_sheet},
    payload::{
        entities::{
            Account, AddrSimple, Bank, CodeDesc, EnterpriseCode, Identification, Individual,
            License, Occupation, Organization,
        },
        section3::Section3,
    },
    template::cell_value_from_key,
    utils::datetime::ConvertDateFormat,
};

impl Section3 {
    pub fn from_excel<RS>(
        workbook: &mut calamine::Xlsx<RS>,
        _file_path: &std::path::Path,
    ) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Self::_from_excel(workbook)
            .with_context(|| format!("Lỗi xử lý dữ liệu Phần II - Thông tin khách hàng"))
    }

    fn _from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(Self {
            related_individuals: Individual::from_excel_related_party(workbook)?,
            related_organizations: Organization::from_excel_related_party(workbook)?,
            additional_info: cell_value_from_key("Phần II: Thông tin bổ sung", workbook)?.into(),
        })
    }
}

impl Individual {
    pub fn from_excel_related_party<RS>(
        workbook: &mut calamine::Xlsx<RS>,
    ) -> anyhow::Result<Option<Vec<Self>>>
    where
        RS: Read + Seek,
    {
        Self::_from_excel_related_party(workbook)
            .with_context(|| format!("Lỗi xử lý tại sheet `Phần II. KHCN`"))
    }

    fn _from_excel_related_party<RS>(
        workbook: &mut calamine::Xlsx<RS>,
    ) -> anyhow::Result<Option<Vec<Self>>>
    where
        RS: Read + Seek,
    {
        let sheet_key = "Phần III. CN liên quan";
        let accounts = Account::from_excel_related_party(workbook)?;
        let (rows, col_map, base_coord) = read_table_from_sheet(workbook, sheet_key)?;

        let persons = rows
            .into_iter()
            .map(|curr_row| -> anyhow::Result<Individual> {
                let cell_value_func =
                    |col_name: &str| get_cell_value(col_name, &col_map, base_coord, &curr_row);

                let id_number =
                    cell_value_func("CMND/CCCD/Hộ chiếu/Định danh cá nhân")?.unwrap_or_default();

                let individual = Individual {
                    id: id_number.clone().into(),
                    existing_customer: None,
                    full_name: cell_value_func("Họ và tên")?,
                    date_of_birth: cell_value_func("Ngày tháng năm sinh (dd/mm/yyyy)")?
                        .convert_date_vn_to_iso()?,
                    age_range: cell_value_func("Ngày tháng năm sinh (dd/mm/yyyy)")?
                        .to_age_range_code(),
                    gender: cell_value_func("Giới tính")?.to_gender_code()?.into(),
                    nationality: cell_value_func("Quốc tịch")?.to_country_code()?.into(),
                    occupation: Occupation {
                        occupation_code: cell_value_func("Nghề nghiệp")?
                            .to_occupation_code()?
                            .into(),
                        description: cell_value_func("Nghề nghiệp")?,
                        content: cell_value_func("Nếu Nghề nghiệp Khác")?,
                    }
                    .into(),
                    position: None,
                    permanent_address: Some(AddrSimple {
                        street_address: cell_value_func("Địa chỉ đăng ký thường trú (Số nhà)")?,
                        city_province: cell_value_func("Địa chỉ đăng ký thường trú (Tỉnh/TP)")?,
                        district: cell_value_func("Địa chỉ đăng ký thường trú (Phường/Xã)")?,
                        country: cell_value_func("Địa chỉ đăng ký thường trú (Quốc gia)")?
                            .to_country_code()?
                            .into(),
                        phone: None,
                    }),
                    current_address: Some(AddrSimple {
                        street_address: cell_value_func("Nơi ở hiện tại (Số nhà)")?,
                        city_province: cell_value_func("Nơi ở hiện tại (Tỉnh/TP)")?,
                        district: cell_value_func("Nơi ở hiện tại (Phường/Xã)")?,
                        country: cell_value_func("Nơi ở hiện tại (Quốc gia)")?
                            .to_country_code()?
                            .into(),
                        phone: None,
                    }),
                    identifications: Some(vec![Identification {
                        id_type: cell_value_func("Loại định danh")?
                            .to_personal_id_code()?
                            .into(),
                        id_number: cell_value_func("CMND/CCCD/Hộ chiếu/Định danh cá nhân")?,
                        issue_date: cell_value_func("Ngày cấp (dd/mm/yyyy)")?
                            .convert_date_vn_to_iso()?,
                        issuing_authority: cell_value_func("Cơ quan cấp")?,
                        expiry_date: cell_value_func("Ngày hết hạn (dd/mm/yyyy)")?
                            .convert_date_vn_to_iso()?,
                        place_of_issue: cell_value_func("Nơi cấp")?,
                    }]),
                    phone_number: cell_value_func("Số điện thoại")?,
                    education_level: None,
                    email: None,
                    accounts: accounts.get(&id_number).cloned(),
                };

                Ok(individual)
            })
            .enumerate()
            .fold(
                anyhow::Result::<Vec<Individual>>::Ok(vec![]),
                |final_result, element| {
                    let mut final_result = final_result?;

                    let (n_row, current_result) = element;
                    let err_context = || {
                        format!(
                            "Lỗi dữ liệu khi xử lý dòng số {}",
                            n_row + base_coord.0 as usize + 2
                        )
                    };
                    let current_result = current_result.with_context(err_context)?;

                    final_result.push(current_result);
                    return Ok(final_result);
                },
            )?
            .into();

        Ok(persons)
    }
}

impl Organization {
    pub fn from_excel_related_party<RS>(
        workbook: &mut calamine::Xlsx<RS>,
    ) -> anyhow::Result<Option<Vec<Self>>>
    where
        RS: Read + Seek,
    {
        Self::_from_excel_related_party(workbook)
            .with_context(|| format!("Lỗi xử lý dữ liệu tại sheet `Phần II. KHTC`"))
    }

    fn _from_excel_related_party<RS>(
        workbook: &mut calamine::Xlsx<RS>,
    ) -> anyhow::Result<Option<Vec<Self>>>
    where
        RS: Read + Seek,
    {
        let accounts = Account::from_excel_related_party(workbook)?;

        let sheet_key = "Phần III. TC liên quan";
        let (rows, col_map, base_coord) = read_table_from_sheet(workbook, sheet_key)?;

        let orgs = rows
            .into_iter()
            .map(|curr_row| -> anyhow::Result<Organization> {
                let cell_value_func =
                    |col_name: &str| get_cell_value(col_name, &col_map, base_coord, &curr_row);

                let enterprise_code =
                    cell_value_func("MS doanh nghiệp/MS thuế")?.unwrap_or_default();

                let org = Organization {
                    id: enterprise_code.clone().into(),
                    existing_customer: None,
                    name: cell_value_func("Tên đầy đủ của tổ chức")?,
                    foreign_name: cell_value_func("Tên tiếng nước ngoài (nếu có)")?,
                    short_name: cell_value_func("Tên viết tắt (nếu có)")?,
                    organization_type: CodeDesc {
                        type_code: None,
                        description: None,
                    }
                    .into(),
                    address: AddrSimple {
                        street_address: cell_value_func("Số nhà")?,
                        district: cell_value_func("Phường/Xã")?,
                        city_province: cell_value_func("Tỉnh/TP")?,
                        country: cell_value_func("Quốc gia")?
                            .unwrap_or_default()
                            .to_country_code()?
                            .into(),
                        phone: cell_value_func("Số điện thoại")?,
                    }
                    .into(),
                    establishment_license: License {
                        license_number: cell_value_func("Giấy phép thành lập số")?,
                        issue_date: cell_value_func("Ngày cấp giấy phép (dd/mm/yyyy)")?
                            .convert_date_vn_to_iso()?,
                        issue_place: cell_value_func("Nơi cấp giấy phép")?,
                    }
                    .into(),
                    enterprise_code: EnterpriseCode {
                        code: cell_value_func("MS doanh nghiệp/MS thuế")?,
                        issue_date: cell_value_func("Ngày cấp MST (dd/mm/yyyy)")?
                            .convert_date_vn_to_iso()?,
                        issue_place: cell_value_func("Quốc gia cấp MST")?
                            .to_country_code()?
                            .into(),
                    }
                    .into(),
                    business_sector: cell_value_func("Ngành nghề kinh doanh chính")?,
                    phone_number: cell_value_func("Số điện thoại")?,
                    website: cell_value_func("Địa chỉ trang thông tin điện tử của doanh nghiệp")?,
                    accounts: accounts.get(&enterprise_code).cloned().into(),
                    representatives: None,
                };
                Ok(org)
            })
            .enumerate()
            .fold(
                anyhow::Result::<Vec<Organization>>::Ok(vec![]),
                |final_result, result| {
                    let mut final_result = final_result?;

                    let (n_row, current_result) = result;
                    let err_context = || {
                        format!(
                            "Lỗi dữ liệu khi xử lý dòng số {}",
                            n_row + base_coord.0 as usize + 2
                        )
                    };
                    let current_result = current_result.with_context(err_context)?;

                    final_result.push(current_result);
                    Ok(final_result)
                },
            )?;

        Ok(orgs.into())
    }
}

impl Account {
    pub fn from_excel_related_party<RS>(
        workbook: &mut calamine::Xlsx<RS>,
    ) -> anyhow::Result<HashMap<String, Vec<Self>>>
    where
        RS: Read + Seek,
    {
        Self::_from_excel_related_party(workbook)
            .with_context(|| format!("Lỗi xử lý dữ liệu tại sheet `Phần II. Tài khoản`"))
    }

    fn _from_excel_related_party<RS>(
        workbook: &mut calamine::Xlsx<RS>,
    ) -> anyhow::Result<HashMap<String, Vec<Self>>>
    where
        RS: Read + Seek,
    {
        let sheet_key = "Phần III. Tài khoản liên quan";
        let (rows, col_map, base_coord) = read_table_from_sheet(workbook, sheet_key)?;

        let accounts = rows
            .into_iter()
            .map(|curr_row| -> anyhow::Result<_> {
                let cell_value_func =
                    |col_name: &str| get_cell_value(col_name, &col_map, base_coord, &curr_row);

                let cif_value = cell_value_func("Số giấy tờ")?.unwrap_or_default();

                let account = Account {
                    account_number: cell_value_func("Số tài khoản")?,
                    bank: Some(Bank {
                        bank_name: cell_value_func("Tên Ngân hàng")?,
                        bank_code: cell_value_func("Mã Ngân hàng")?
                            .map(|v| v.split("-").next().unwrap_or_default().trim().to_string()),
                    }),
                    currency_type: cell_value_func("Loại tiền")?.to_currency_code()?.into(),
                    account_type: cell_value_func("Loại TK")?.to_account_type_code()?.into(),
                    open_date: cell_value_func("Ngày mở")?.convert_date_vn_to_iso()?,
                    status: cell_value_func("Trạng thái")?
                        .to_account_status_code()?
                        .into(),
                    authorized_persons: None,
                };

                Ok((cif_value, account))
            })
            .enumerate()
            .fold(
                anyhow::Result::<HashMap<String, Vec<Account>>>::Ok(Default::default()),
                |final_result, element| {
                    let mut final_result = final_result?;

                    let (n_row, current_result) = element;
                    let err_context = || {
                        format!(
                            "Lỗi dữ liệu khi xử lý dòng số {}",
                            n_row + base_coord.0 as usize + 2
                        )
                    };
                    let (cif, account) = current_result.with_context(err_context)?;

                    final_result.entry(cif).or_default().push(account);
                    Ok(final_result)
                },
            )?;

        Ok(accounts)
    }
}
