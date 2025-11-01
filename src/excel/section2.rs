use std::{
    collections::HashMap,
    io::{Read, Seek},
};

use anyhow::Context;
use calamine::{DataType, Reader};

use crate::{
    codes::{
        account_status::AccountStatusCode, account_type::AccountTypeCode, age_range::AgeRangeCode,
        corporate_type::CorporateTypeCode, country::CountryCode, gender::GenderCode,
        occupation::OccupationCode, personal_id::PersonalIdCode,
    },
    payload::{
        entities::{
            Account, AddrSimple, Bank, BeneficialOwners, BenefitGroup, CodeDesc, EnterpriseCode,
            Identification, Individual, IndividualLink, License, Occupation, Organization,
            OrganizationLink, PersonRef, Representative,
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
        Self::_from_excel(workbook).with_context(|| format!("Lỗi xử lý dữ liệu Phần II"))
    }

    fn _from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
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

pub fn read_table_from_sheet<RS>(
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

pub fn get_cell_value(
    col_name: &str,
    col_map: &HashMap<String, String>,
    base_coord: (u32, u32),
    curr_row: &Vec<String>,
) -> anyhow::Result<Option<String>> {
    let col_no = col_map
        .get(col_name)
        .cloned()
        .with_context(|| format!("Không tìm thấy cột {}", col_name))?;

    let col_idx = col_name_to_index(&col_no, base_coord.into())
        .with_context(|| format!("Không tìm thấy cột {} {}", col_no, col_name))?;

    let value = curr_row
        .get(col_idx as usize)
        .map(|s| match s.is_empty() {
            true => None,
            false => Some(s.clone()),
        })
        .flatten();

    Ok(value)
}

impl Individual {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Option<Vec<Self>>>
    where
        RS: Read + Seek,
    {
        Self::_from_excel(workbook).with_context(|| format!("Lỗi xử lý tại sheet `Phần II. KHCN`"))
    }

    fn _from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Option<Vec<Self>>>
    where
        RS: Read + Seek,
    {
        let accounts = Account::from_excel(workbook)?;
        let (rows, col_map, base_coord) = read_table_from_sheet(workbook, "Phần II. KHCN")?;

        let persons = rows
            .into_iter()
            .map(|curr_row| -> anyhow::Result<Individual> {
                let cell_value_func =
                    |col_name: &str| get_cell_value(col_name, &col_map, base_coord, &curr_row);

                let cif_value = cell_value_func("CIF")?.unwrap_or_default();

                let individual = Individual {
                    id: cif_value.clone().into(),
                    existing_customer: if cif_value.clone().is_empty() {
                        None
                    } else {
                        "1".to_string().into()
                    },
                    full_name: cell_value_func("Tên khách hàng")?,
                    date_of_birth: cell_value_func("Ngày tháng năm sinh (dd/mm/yyyy)")?
                        .convert_date_vn_to_iso(),
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
                            .convert_date_vn_to_iso(),
                        issuing_authority: cell_value_func("Cơ quan cấp")?,
                        expiry_date: cell_value_func("Ngày hết hạn (dd/mm/yyyy)")?
                            .convert_date_vn_to_iso(),
                        place_of_issue: cell_value_func("Nơi cấp")?,
                    }]),
                    phone_number: cell_value_func("Số điện thoại")?,
                    education_level: None,
                    email: cell_value_func("Email")?,
                    accounts: accounts.get(&cif_value).cloned(),
                };

                Ok(individual)
            })
            .enumerate()
            .map(|(n_row, result)| -> anyhow::Result<Individual> {
                match result {
                    Ok(person) => Ok(person),
                    Err(e) => {
                        let err_msg =
                            format!("Lỗi dữ liệu khi xử lý dòng số {}: {:?}", n_row + 1, e);
                        Err(e).context(err_msg)
                    }
                }
            })
            .fold(
                anyhow::Result::<Vec<Individual>>::Ok(vec![]),
                |final_result, result| {
                    let mut current_result = match final_result {
                        Ok(r) => r,
                        Err(e) => return Err(e),
                    };

                    match result {
                        Ok(person) => current_result.push(person),
                        Err(e) => return Err(e),
                    };

                    return Ok(current_result);
                },
            )?
            .into();

        Ok(persons)
    }
}

impl Organization {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Option<Vec<Self>>>
    where
        RS: Read + Seek,
    {
        Self::_from_excel(workbook)
            .with_context(|| format!("Lỗi xử lý dữ liệu tại sheet `Phần II. KHTC`"))
    }

    fn _from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Option<Vec<Self>>>
    where
        RS: Read + Seek,
    {
        let accounts = Account::from_excel(workbook)?;
        let rep_persons = Representative::from_excel(workbook)?;

        let sheet_key = "Phần II. KHTC";
        let (rows, col_map, base_coord) = read_table_from_sheet(workbook, sheet_key)?;

        let orgs = rows
            .into_iter()
            .map(|curr_row| -> anyhow::Result<Organization> {
                let cell_value_func =
                    |col_name: &str| get_cell_value(col_name, &col_map, base_coord, &curr_row);

                let cif_value = cell_value_func("CIF")?.unwrap_or_default();

                let org = Organization {
                    id: cif_value.clone().into(),
                    existing_customer: if cif_value.clone().is_empty() {
                        None
                    } else {
                        "1".to_string().into()
                    },
                    name: cell_value_func("Tên khách hàng")?,
                    foreign_name: None,
                    short_name: None,
                    organization_type: CodeDesc {
                        type_code: cell_value_func("Loại hình tổ chức")?
                            .to_corporate_type_code()?
                            .into(),
                        description: cell_value_func("Loại hình tổ chức nếu chọn Khác")?,
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
                            .convert_date_vn_to_iso(),
                        issue_place: cell_value_func("Nơi cấp giấy phép")?,
                    }
                    .into(),
                    enterprise_code: EnterpriseCode {
                        code: cell_value_func("MS doanh nghiệp/MS thuế")?,
                        issue_date: cell_value_func("Ngày cấp MST (dd/mm/yyyy)")?
                            .convert_date_vn_to_iso(),
                        issue_place: cell_value_func("Quốc gia cấp MST")?
                            .to_country_code()?
                            .into(),
                    }
                    .into(),
                    business_sector: cell_value_func("Ngành nghề kinh doanh chính")?,
                    phone_number: cell_value_func("Số điện thoại")?,
                    website: cell_value_func("Địa chỉ trang thông tin điện tử của doanh nghiệp")?,
                    accounts: accounts.get(&cif_value).cloned().into(),
                    representatives: rep_persons.get(&cif_value).cloned().into(),
                };
                Ok(org)
            })
            .enumerate()
            .fold(
                anyhow::Result::<Vec<Organization>>::Ok(vec![]),
                |final_result, result| {
                    let mut final_result = final_result?;

                    let (n_row, current_result) = result;
                    let err_context = || format!("Lỗi dữ liệu khi xử lý dòng số {}", n_row + 1);
                    let current_result = current_result.with_context(err_context)?;

                    final_result.push(current_result);
                    Ok(final_result)
                },
            )?;

        Ok(orgs.into())
    }
}

impl Account {
    pub fn from_excel<RS>(
        workbook: &mut calamine::Xlsx<RS>,
    ) -> anyhow::Result<HashMap<String, Vec<Self>>>
    where
        RS: Read + Seek,
    {
        Self::_from_excel(workbook)
            .with_context(|| format!("Lỗi xử lý dữ liệu tại sheet `Phần II. Tài khoản`"))
    }

    fn _from_excel<RS>(
        workbook: &mut calamine::Xlsx<RS>,
    ) -> anyhow::Result<HashMap<String, Vec<Self>>>
    where
        RS: Read + Seek,
    {
        let sheet_key = "Phần II. Tài khoản";
        let (rows, col_map, base_coord) = read_table_from_sheet(workbook, sheet_key)?;

        let accounts = rows
            .into_iter()
            .map(|curr_row| -> anyhow::Result<_> {
                let cell_value_func =
                    |col_name: &str| get_cell_value(col_name, &col_map, base_coord, &curr_row);

                let cif_value = cell_value_func("CIF")?.unwrap_or_default();

                let account = Account {
                    account_number: cell_value_func("Số tài khoản")?,
                    bank: Some(Bank {
                        bank_name: cell_value_func("Tên Ngân hàng")?,
                        bank_code: cell_value_func("Mã Ngân hàng")?
                            .map(|v| v.split("-").next().unwrap_or_default().trim().to_string()),
                    }),
                    currency_type: cell_value_func("Loại tiền")?
                        .map(|v| v.split("-").next().unwrap_or_default().trim().to_string()),
                    account_type: cell_value_func("Loại TK")?.to_account_type_code()?.into(),
                    open_date: cell_value_func("Ngày mở")?.convert_date_vn_to_iso(),
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
                    let err_context = || format!("Lỗi dữ liệu khi xử lý dòng số {}", n_row + 1);
                    let (cif, account) = current_result.with_context(err_context)?;

                    final_result.entry(cif).or_default().push(account);
                    Ok(final_result)
                },
            )?;

        Ok(accounts)
    }
}

impl Representative {
    pub fn from_excel<RS>(
        workbook: &mut calamine::Xlsx<RS>,
    ) -> anyhow::Result<HashMap<String, Vec<Self>>>
    where
        RS: Read + Seek,
    {
        Self::_from_excel(workbook)
            .with_context(|| format!("Lỗi xử lý dữ liệu tại sheet `Phần II. Người đại diện`"))
    }

    fn _from_excel<RS>(
        workbook: &mut calamine::Xlsx<RS>,
    ) -> anyhow::Result<HashMap<String, Vec<Self>>>
    where
        RS: Read + Seek,
    {
        let sheet_key = "Phần II. Người đại diện";
        let (rows, col_map, base_coord) = read_table_from_sheet(workbook, sheet_key)?;

        let representatives = rows
            .into_iter()
            .map(|curr_row| -> anyhow::Result<(String, Representative)> {
                let cell_value_func =
                    |col_name: &str| get_cell_value(col_name, &col_map, base_coord, &curr_row);

                let cif_value = cell_value_func("CIF")?.unwrap_or_default();

                let rep = Representative {
                    id: cell_value_func("CMND/CCCD/Hộ chiếu/Định danh cá nhân")?,
                    full_name: cell_value_func("Họ và tên")?,
                    date_of_birth: cell_value_func("Ngày sinh")?.convert_date_vn_to_iso(),
                    occupation: Occupation {
                        occupation_code: cell_value_func("Nghề nghiệp")?
                            .to_occupation_code()?
                            .into(),
                        description: cell_value_func("Nghề nghiệp")?,
                        content: cell_value_func("Nếu Nghề nghiệp Khác")?,
                    }
                    .into(),
                    position: cell_value_func("Chức vụ/vị trí việc làm")?,
                    permanent_address: AddrSimple {
                        street_address: cell_value_func("Địa chỉ đăng ký thường trú (Số nhà)")?,
                        city_province: cell_value_func("Địa chỉ đăng ký thường trú (Tỉnh/TP)")?,
                        district: cell_value_func("Địa chỉ đăng ký thường trú (Phường/Xã)")?,
                        country: cell_value_func("Địa chỉ đăng ký thường trú (Quốc gia)")?
                            .to_country_code()?
                            .into(),
                        phone: None,
                    }
                    .into(),
                    current_address: AddrSimple {
                        street_address: cell_value_func("Nơi ở hiện tại (Số nhà)")?,
                        city_province: cell_value_func("Nơi ở hiện tại (Tỉnh/TP)")?,
                        district: cell_value_func("Nơi ở hiện tại (Phường/Xã)")?,
                        country: cell_value_func("Nơi ở hiện tại (Quốc gia)")?
                            .to_country_code()?
                            .into(),
                        phone: None,
                    }
                    .into(),
                    phone_number: cell_value_func("Điện thoại liên lạc")?,
                    nationality: cell_value_func("Quốc tịch")?.to_country_code()?.into(),
                    identifications: Some(vec![Identification {
                        id_type: cell_value_func("Loại định danh")?
                            .to_personal_id_code()?
                            .into(),
                        id_number: cell_value_func("CMND/CCCD/Hộ chiếu/Định danh cá nhân")?,
                        issue_date: cell_value_func("Ngày cấp (dd/mm/yyyy)")?
                            .convert_date_vn_to_iso(),
                        issuing_authority: cell_value_func("Cơ quan cấp")?,
                        expiry_date: None,
                        place_of_issue: cell_value_func("Nơi cấp")?,
                    }]),
                };

                Ok((cif_value, rep))
            })
            .enumerate()
            .fold(
                anyhow::Result::<HashMap<String, Vec<Representative>>>::Ok(Default::default()),
                |final_result, element| {
                    let mut final_result = final_result?;

                    let (n_row, element) = element;
                    let err_context = || format!("Lỗi dữ liệu khi xử lý dòng số {}", n_row + 1);
                    let (cif, rep) = element.with_context(err_context)?;
                    final_result.entry(cif).or_default().push(rep);
                    Ok(final_result)
                },
            )?;

        Ok(representatives)
    }
}

impl BeneficialOwners {
    fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Option<Self>>
    where
        RS: Seek + Read,
    {
        Self::_from_excel(workbook)
            .with_context(|| format!("Lỗi xử lý dữ liệu chủ sở hữu hưởng lợi tại Phần II"))
    }

    fn _from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Option<Self>>
    where
        RS: Seek + Read,
    {
        let other_owners = other_owners_from_excel(workbook)?;
        let other_owners_list = other_owners.values().cloned().flatten().collect::<Vec<_>>();

        let individuals = Individual::from_excel(workbook)?.unwrap_or_default();
        let individual_links = individuals
            .into_iter()
            .map(|person| {
                let cif = person.id;
                let full_name = person.full_name;
                let id_number = person
                    .identifications
                    .as_ref()
                    .and_then(|ids| ids.first())
                    .and_then(|id| id.id_number.clone());

                let benefit_group = match &cif {
                    None => None,
                    Some(cif_value) => other_owners.get(cif_value).map(|group| {
                        let other_group = group
                            .iter()
                            .map(|person| PersonRef {
                                id: person.id.clone(),
                                full_name: person.full_name.clone(),
                                id_number: person
                                    .identifications
                                    .as_ref()
                                    .and_then(|ids| ids.first())
                                    .and_then(|id| id.id_number.clone()),
                            })
                            .collect::<Vec<_>>();

                        BenefitGroup {
                            main_group: None,
                            other_group: Some(other_group),
                        }
                    }),
                };

                IndividualLink {
                    id: cif,
                    name: full_name,
                    id_number,
                    is_principal: true.into(),
                    benefit_group: benefit_group,
                }
            })
            .collect::<Vec<_>>();

        let orgs = Organization::from_excel(workbook)?.unwrap_or_default();
        let representatives = Representative::from_excel(workbook)?;

        let organization_links = orgs
            .into_iter()
            .map(|org| {
                let cif = org.id;
                let name = org.name;
                let id_number = org.enterprise_code.as_ref().and_then(|ec| ec.code.clone());

                let benefit_group: Option<BenefitGroup> = match &cif {
                    None => None,
                    Some(cif_value) => {
                        let main_group = representatives.get(cif_value).map(|reps| {
                            reps.iter()
                                .map(|rep| PersonRef {
                                    id: rep.id.clone(),
                                    full_name: rep.full_name.clone(),
                                    id_number: rep
                                        .identifications
                                        .as_ref()
                                        .and_then(|ids| ids.first())
                                        .and_then(|id| id.id_number.clone()),
                                })
                                .collect::<Vec<_>>()
                        });

                        let other_group = other_owners.get(cif_value).map(|group| {
                            group
                                .iter()
                                .map(|person| PersonRef {
                                    id: person.id.clone(),
                                    full_name: person.full_name.clone(),
                                    id_number: person
                                        .identifications
                                        .as_ref()
                                        .and_then(|ids| ids.first())
                                        .and_then(|id| id.id_number.clone()),
                                })
                                .collect::<Vec<_>>()
                        });

                        BenefitGroup {
                            main_group: main_group,
                            other_group: other_group,
                        }
                        .into()
                    }
                };

                OrganizationLink {
                    id: cif,
                    name: name,
                    id_number: id_number,
                    benefit_group: benefit_group,
                }
            })
            .collect::<Vec<_>>();

        Ok(BeneficialOwners {
            other_owners: other_owners_list.into(),
            individual_links: individual_links.into(),
            organization_links: organization_links.into(),
        }
        .into())
    }
}

fn other_owners_from_excel<RS>(
    workbook: &mut calamine::Xlsx<RS>,
) -> anyhow::Result<HashMap<String, Vec<Individual>>>
where
    RS: Seek + Read,
{
    _other_owners_from_excel(workbook)
        .with_context(|| format!("Lỗi khi xử lý dữ liệu tại sheet `Phần II. CSHHL khác`"))
}

fn _other_owners_from_excel<RS>(
    workbook: &mut calamine::Xlsx<RS>,
) -> anyhow::Result<HashMap<String, Vec<Individual>>>
where
    RS: Seek + Read,
{
    let sheet_key = "Phần II. CSHHL khác";
    let (rows, col_map, base_coord) = read_table_from_sheet(workbook, sheet_key)?;

    let beneficiaries = rows
        .into_iter()
        .map(|curr_row| -> anyhow::Result<(String, Individual)> {
            let cell_value_func =
                |col_name: &str| get_cell_value(col_name, &col_map, base_coord, &curr_row);

            let cif_value = cell_value_func("CIF")?.unwrap_or_default();

            let rep = Individual {
                existing_customer: None,
                id: cif_value.clone().into(),
                full_name: cell_value_func("Họ và tên")?,
                date_of_birth: cell_value_func("Ngày sinh")?.convert_date_vn_to_iso(),
                age_range: None,
                gender: cell_value_func("Giới tính")?.to_gender_code()?.into(),
                nationality: cell_value_func("Quốc tịch")?.to_country_code()?.into(),
                occupation: Occupation {
                    occupation_code: cell_value_func("Nghề nghiệp")?.to_occupation_code()?.into(),
                    description: cell_value_func("Nghề nghiệp")?,
                    content: cell_value_func("Nếu Nghề nghiệp Khác")?,
                }
                .into(),
                position: cell_value_func("Chức vụ/vị trí việc làm")?,
                permanent_address: AddrSimple {
                    street_address: cell_value_func("Địa chỉ đăng ký thường trú (Số nhà)")?,
                    city_province: cell_value_func("Địa chỉ đăng ký thường trú (Tỉnh/TP)")?,
                    district: cell_value_func("Địa chỉ đăng ký thường trú (Phường/Xã)")?,
                    country: cell_value_func("Địa chỉ đăng ký thường trú (Quốc gia)")?
                        .to_country_code()?
                        .into(),
                    phone: None,
                }
                .into(),
                current_address: AddrSimple {
                    street_address: cell_value_func("Nơi ở hiện tại (Số nhà)")?,
                    city_province: cell_value_func("Nơi ở hiện tại (Tỉnh/TP)")?,
                    district: cell_value_func("Nơi ở hiện tại (Phường/Xã)")?,
                    country: cell_value_func("Nơi ở hiện tại (Quốc gia)")?
                        .to_country_code()?
                        .into(),
                    phone: None,
                }
                .into(),
                phone_number: cell_value_func("Điện thoại liên lạc")?,
                identifications: Some(vec![Identification {
                    id_type: cell_value_func("Loại định danh")?
                        .to_personal_id_code()?
                        .into(),
                    id_number: cell_value_func("CMND/CCCD/Hộ chiếu/Định danh cá nhân")?,
                    issue_date: cell_value_func("Ngày cấp (dd/mm/yyyy)")?.convert_date_vn_to_iso(),
                    issuing_authority: cell_value_func("Cơ quan cấp")?,
                    expiry_date: None,
                    place_of_issue: cell_value_func("Nơi cấp")?,
                }]),
                education_level: None,
                email: None,
                accounts: None,
            };

            Ok((cif_value, rep))
        })
        .enumerate()
        .fold(
            anyhow::Result::<HashMap<String, Vec<Individual>>>::Ok(Default::default()),
            |acc, element| {
                let mut result = acc?;

                let (n_row, element) = element;
                let err_context = || format!("Lỗi dữ liệu khi xử lý dòng số {}", n_row + 1);
                let (cif, rep) = element.with_context(err_context)?;

                result.entry(cif).or_default().push(rep);
                Ok(result)
            },
        )?;

    Ok(beneficiaries)
}
