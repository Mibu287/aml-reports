use std::{
    collections::{HashMap, HashSet},
    io::{Read, Seek},
};

use anyhow::{Context, Ok};
use calamine::{DataType, Reader};

use crate::{
    codes::currency::CurrencyCode,
    payload::{
        entities::{Account, Individual, Organization},
        section4::{
            AmountEntry, Analysis, Clause, ConclusionEntry, FlowEntryIn, FlowEntryOut, LegalBasis,
            MoneyFlow, ReportType, Section4, SuspiciousIndicator, TimeRange, TransactionInfo,
        },
    },
    template::{
        cell_value_from_key, legal_basis_mapping_from_key, mapping_from_key, table_config_from_key,
        value_list_from_key,
    },
    utils::{
        datetime::ConvertDateFormat,
        excel::{col_name_to_index, read_cell_value},
    },
};

impl Section4 {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(Section4 {
            report_type: ReportType::from_excel(workbook)?.into(),
            transaction_info: TransactionInfo::from_excel(workbook)?.into(),
            analysis: Analysis::from_excel(workbook)?.into(),
            conclusions: ConclusionEntry::from_excel(workbook)?.into(),
            detection_date: cell_value_from_key(
                "Phần IV: Ngày phát hiện giao dịch đáng ngờ",
                workbook,
            )
            .ok()
            .convert_date_vn_to_iso()?,
        })
    }
}

impl ReportType {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        let sheet_name = cell_value_from_key("Phần IV: Thông tin về giao dịch đáng ngờ", workbook)?;
        let checked_box = cell_value_from_key("Dấu tick", workbook)?;
        let range = workbook.worksheet_range(&sheet_name)?;

        let selection = range
            .rows()
            .into_iter()
            .map(|row| {
                let key = row
                    .get(0)
                    .map(|c| c.get_string().unwrap_or_default().trim().to_string())
                    .unwrap_or(Default::default());

                let value = row
                    .get(1)
                    .map(|c| c.get_string().unwrap_or_default().trim().to_string())
                    .map(|c| c == checked_box)
                    .unwrap_or_default();

                (key, value)
            })
            .filter(|(k, v)| !k.is_empty() && *v)
            .collect::<HashMap<_, _>>();

        let report_key = "Phần IV: Loại báo cáo giao dịch đáng ngờ";
        let reports = mapping_from_key(report_key)?
            .into_iter()
            .filter(|(k, _)| selection.get(k).copied().unwrap_or(false))
            .map(|(k, v)| Clause {
                code: k.into(),
                description: v.into(),
            })
            .collect::<Vec<_>>();

        let indicator_key = "Phần IV: Dấu hiệu đáng ngờ";
        let indicators = mapping_from_key(indicator_key)?
            .into_iter()
            .filter(|(k, _)| selection.get(k).copied().unwrap_or(false))
            .map(|(k, v)| SuspiciousIndicator {
                code: k.into(),
                description: v.into(),
                other_content: None,
            })
            .collect::<Vec<_>>();

        Ok(Self {
            clauses: reports.into(),
            suspicious_indicators: indicators.into(),
        })
    }
}

impl Analysis {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        let sheet_key = "Phần IV: Thông tin về giao dịch đáng ngờ";
        let sheet_name = cell_value_from_key(sheet_key, workbook)?;

        let detail_key = "Phần IV: Mô tả, phân tích chi tiết";
        let detail_analysis = value_list_from_key(detail_key)?
            .into_iter()
            .map(|cell_name| read_cell_value(workbook, &sheet_name, &cell_name).unwrap_or_default())
            .filter(|v| !v.is_empty())
            .fold(String::new(), |mut acc, s| {
                if !acc.is_empty() {
                    acc.push_str("\n");
                }
                acc.push_str(&s);
                acc
            });

        Ok(Analysis {
            detail: detail_analysis.into(),
            legal_bases: LegalBasis::from_excel(workbook)?.into(),
        })
    }
}

impl LegalBasis {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Vec<Self>>
    where
        RS: Seek + Read,
    {
        let sheet_key = "Phần IV: Thông tin về giao dịch đáng ngờ";
        let sheet_name = cell_value_from_key(sheet_key, workbook)?;

        let legal_basis_key = "Phần IV: Cơ sở hợp lý để nghi ngờ";

        let legal_basis = legal_basis_mapping_from_key(legal_basis_key)?
            .into_iter()
            .map(|(field, basis)| {
                let notice_number = basis
                    .document_number
                    .and_then(|cell_name| read_cell_value(workbook, &sheet_name, &cell_name).ok())
                    .map(|s| if s.is_empty() { None } else { Some(s) })
                    .flatten();

                let basis_text = basis
                    .basis
                    .and_then(|cell_name| read_cell_value(workbook, &sheet_name, &cell_name).ok())
                    .map(|s| if s.is_empty() { None } else { Some(s) })
                    .flatten();

                LegalBasis {
                    report_type: field.into(),
                    notice_number: notice_number,
                    basis: basis_text,
                }
            })
            .collect::<Vec<_>>();

        Ok(legal_basis)
    }
}

impl ConclusionEntry {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Vec<Self>>
    where
        RS: Seek + Read,
    {
        let sheet_key = "Phần IV: Thông tin về giao dịch đáng ngờ";
        let sheet_name = cell_value_from_key(sheet_key, workbook)?;

        let checked_box = cell_value_from_key("Dấu tick", workbook)?;
        let range = workbook.worksheet_range(&sheet_name)?;

        let selection = range
            .rows()
            .into_iter()
            .map(|row| {
                let crime_code = row
                    .get(0)
                    .map(|c| c.get_string().unwrap_or_default().trim().to_string())
                    .unwrap_or(Default::default());

                let is_selected = row
                    .get(1)
                    .map(|c| c.get_string().unwrap_or_default().trim().to_string())
                    .map(|c| c == checked_box)
                    .unwrap_or_default();

                let other_content = row
                    .get(2)
                    .map(|c| c.get_string().unwrap_or_default().trim().to_string())
                    .unwrap_or(Default::default());

                (crime_code, (is_selected, other_content))
            })
            .filter(|(crime_code, (is_selected, other_content))| {
                !crime_code.is_empty() && (*is_selected || !other_content.is_empty())
            })
            .collect::<HashMap<_, _>>();

        let conclusion_key =
            "Phần IV: Nhận định về loại tội phạm có thể liên quan đến giao dịch đáng ngờ";

        let conclusions = mapping_from_key(conclusion_key)?
            .into_iter()
            .filter(|(k, _)| selection.get(k).map(|(v, _)| v).copied().unwrap_or(false))
            .map(|(crime_code, crime_desc)| {
                let crime_desc_key = format!("{}_desc", crime_code);
                let other_content = selection.get(&crime_desc_key).map(|(_, v)| v).cloned();
                ConclusionEntry {
                    crime_code: crime_code.into(),
                    description: crime_desc.into(),
                    other_content: other_content,
                }
            })
            .collect::<Vec<_>>();

        Ok(conclusions)
    }
}

impl TransactionInfo {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        let checked_box = cell_value_from_key("Dấu tick", workbook)?;
        let status_value =
            cell_value_from_key("Phần IV: Trạng thái của giao dịch đáng ngờ", workbook)?;
        let status = match status_value == checked_box {
            true => "1".to_string().into(),
            false => None,
        };

        let from_date = cell_value_from_key(
            "Phần IV: Thông tin về giao dịch đáng ngờ - Từ ngày",
            workbook,
        )?
        .convert_date_vn_to_iso()?;

        let to_date = cell_value_from_key(
            "Phần IV: Thông tin về giao dịch đáng ngờ - Đến ngày",
            workbook,
        )?
        .convert_date_vn_to_iso()?;

        let moneyflow_details = MoneyFlow::from_excel(workbook)?;

        let amount_by_currency = moneyflow_details
            .iter()
            .map(|flow| {
                let inflows = flow
                    .inflows
                    .iter()
                    .map(|f| {
                        f.iter()
                            .map(|e| (e.currency.as_ref(), e.total_amount.as_ref()))
                    })
                    .flatten();

                let outflows = flow
                    .outflows
                    .iter()
                    .map(|f| {
                        f.iter()
                            .map(|e| (e.currency.as_ref(), e.total_amount.as_ref()))
                    })
                    .flatten();

                let all_flows = inflows.chain(outflows);
                all_flows
            })
            .flatten()
            .fold(
                HashMap::<String, f64>::new(),
                |mut acc, (currency_opt, amount_opt)| {
                    let currency = currency_opt.cloned().unwrap_or_default();
                    let original_amount = amount_opt
                        .cloned()
                        .unwrap_or_default()
                        .parse::<f64>()
                        .ok()
                        .unwrap_or(0.0);

                    *acc.entry(currency).or_insert(0.0) += original_amount;
                    acc
                },
            )
            .into_iter()
            .map(|(currency, total_amount)| AmountEntry {
                currency: currency.into(),
                amount: total_amount.into(),
            })
            .collect::<Vec<_>>();

        let total_converted_amount = moneyflow_details
            .iter()
            .map(|flow| {
                let total_in = flow
                    .total_converted_in
                    .as_ref()
                    .map(|v| v.parse::<f64>().ok())
                    .flatten()
                    .unwrap_or_default();

                let total_out = flow
                    .total_converted_out
                    .as_ref()
                    .map(|v| v.parse::<f64>().ok())
                    .flatten()
                    .unwrap_or_default();

                total_in + total_out
            })
            .sum::<f64>();

        Ok(Self {
            status: status,
            time_range: TimeRange {
                from: from_date,
                to: to_date,
            }
            .into(),
            amounts: amount_by_currency.into(),
            total_converted_amount: total_converted_amount.into(),
            money_flows: moneyflow_details.into(),
        })
    }
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

impl MoneyFlow {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Vec<Self>>
    where
        RS: Seek + Read,
    {
        Self::_from_excel(workbook)
            .with_context(|| format!("Lỗi xử lý dữ liệu Phần IV - Thông tin về giao dịch đáng ngờ"))
    }

    fn _from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Vec<Self>>
    where
        RS: Seek + Read,
    {
        let inflow_sheet_key = "Phần IV. Ghi Có";
        let outflow_sheet_key = "Phần IV. Ghi Nợ";

        let bank_accounts = {
            Account::from_excel(workbook)?
                .into_iter()
                .map(|(cif, accounts)| {
                    accounts.into_iter().map(move |account| {
                        let bank_info = account.bank.unwrap_or_default();
                        let account_no = account.account_number.clone().unwrap_or_default();
                        ((cif.clone(), account_no), bank_info)
                    })
                })
                .flatten()
                .collect::<HashMap<_, _>>()
        };

        let customer_infos = {
            let persons = Individual::from_excel(workbook)?.unwrap_or_default();
            let orgs = Organization::from_excel(workbook)?.unwrap_or_default();

            let person_ids = persons.iter().map(|p| {
                let cif = p.id.clone();
                let name = p.full_name.clone().unwrap_or_default();
                let id_number = p
                    .identifications
                    .as_ref()
                    .map(|v| v.first())
                    .flatten()
                    .map(|id| id.id_number.clone())
                    .flatten()
                    .unwrap_or_default();

                (cif, name, id_number)
            });

            let org_ids = orgs.iter().map(|org| {
                let cif = org.id.clone();
                let name = org.name.clone().unwrap_or_default();
                let id_number = org
                    .enterprise_code
                    .clone()
                    .unwrap_or_default()
                    .code
                    .unwrap_or_default();

                (cif, name, id_number)
            });

            #[derive(Clone, Default)]
            struct CustomerInfo {
                name: String,
                id_number: String,
            }

            let customer_infos = person_ids
                .chain(org_ids)
                .filter(|(cif, _, _)| cif.is_some())
                .map(|(cif, name, id_number)| {
                    (
                        cif.unwrap_or_default().to_string(),
                        CustomerInfo { name, id_number },
                    )
                })
                .collect::<HashMap<_, _>>();

            customer_infos
        };

        let (inflow_rows, inflow_columns, inflow_base_coord) =
            read_table_from_sheet(workbook, inflow_sheet_key)?;

        let mut inflow_entries = inflow_rows
            .into_iter()
            .map(
                |curr_row| -> anyhow::Result<(String, String, FlowEntryIn)> {
                    let cell_value_func = |col_name: &str| {
                        get_cell_value(col_name, &inflow_columns, inflow_base_coord, &curr_row)
                    };

                    let cif = cell_value_func("CIF").unwrap_or_default();
                    let account_number = cell_value_func("Số tài khoản").unwrap_or_default();

                    let entry = FlowEntryIn {
                        source_name: cell_value_func("Tên cá nhân/ tổ chức đối ứng"),
                        source_id: cell_value_func("Số CMND/ CCCD/ Hộ chiếu/ định danh cá nhân"),
                        source_account: cell_value_func("Số tài khoản áp dụng cho TH chuyển khoản"),
                        source_bank_name: cell_value_func("Tên ngân hàng chuyển tiền"),
                        source_bank_code: cell_value_func("Mã ngân hàng chuyển tiền"),
                        total_amount: cell_value_func("Tổng số tiền nguyên tệ"),
                        total_converted: cell_value_func("Tổng số tiền quy đổi (VND)"),
                        total_transactions: cell_value_func("Tổng số lượng giao dịch"),
                        tx_from: cell_value_func("Giao dịch từ ngày").convert_date_vn_to_iso()?,
                        tx_to: cell_value_func("Giao dịch đến ngày").convert_date_vn_to_iso()?,
                        currency: cell_value_func("Loại tiền").to_currency_code()?.into(),
                        content: cell_value_func("Tóm tắt nội dung giao dịch"),
                    };

                    Ok((cif, account_number, entry))
                },
            )
            .enumerate()
            .fold(
                anyhow::Result::<HashMap<(String, String), Vec<FlowEntryIn>>>::Ok(
                    Default::default(),
                ),
                |final_result, element| {
                    let mut final_result = final_result?;
                    let (n_row, current_result) = element;
                    let err_context = || {
                        format!(
                            "Lỗi xử lý dữ liệu dòng số {}",
                            n_row + inflow_base_coord.0 as usize + 2
                        )
                    };
                    let (cif, account, entry) = current_result.with_context(err_context)?;
                    final_result
                        .entry((cif, account))
                        .or_insert_with(Vec::new)
                        .push(entry);
                    Ok(final_result)
                },
            )
            .with_context(|| format!("Lỗi xử lý dữ liệu sheet {}", inflow_sheet_key))?;

        let (outflow_rows, outflow_columns, outflow_base_coord) =
            read_table_from_sheet(workbook, outflow_sheet_key)?;

        let mut outflow_entries = outflow_rows
            .into_iter()
            .map(
                |curr_row| -> anyhow::Result<(String, String, FlowEntryOut)> {
                    let cell_value_func = |col_name: &str| {
                        get_cell_value(col_name, &outflow_columns, outflow_base_coord, &curr_row)
                    };

                    let cif = cell_value_func("CIF").unwrap_or_default();
                    let account_number = cell_value_func("Số tài khoản").unwrap_or_default();

                    let entry = FlowEntryOut {
                        dest_name: cell_value_func("Tên cá nhân/ tổ chức đối ứng"),
                        dest_id: cell_value_func("Số CMND/ CCCD/ Hộ chiếu/ định danh cá nhân"),
                        dest_account: cell_value_func("Số tài khoản áp dụng cho TH chuyển khoản"),
                        dest_bank_name: cell_value_func("Tên ngân hàng chuyển tiền"),
                        dest_bank_code: cell_value_func("Mã ngân hàng chuyển tiền"),
                        total_amount: cell_value_func("Tổng số tiền nguyên tệ"),
                        total_converted: cell_value_func("Tổng số tiền quy đổi (VND)"),
                        total_transactions: cell_value_func("Tổng số lượng giao dịch"),
                        tx_from: cell_value_func("Giao dịch từ ngày").convert_date_vn_to_iso()?,
                        tx_to: cell_value_func("Giao dịch đến ngày").convert_date_vn_to_iso()?,
                        currency: cell_value_func("Loại tiền").to_currency_code()?.into(),
                        content: cell_value_func("Tóm tắt nội dung giao dịch"),
                    };

                    Ok((cif, account_number, entry))
                },
            )
            .enumerate()
            .fold(
                anyhow::Result::<HashMap<(String, String), Vec<FlowEntryOut>>>::Ok(
                    Default::default(),
                ),
                |final_result, element| {
                    let mut final_result = final_result?;

                    let (n_row, current_result) = element;
                    let err_context = || {
                        format!(
                            "Lỗi dữ liệu khi xử lý dòng số {}",
                            n_row + outflow_base_coord.0 as usize + 2
                        )
                    };
                    let (cif, account, entry) = current_result.with_context(err_context)?;
                    final_result
                        .entry((cif, account))
                        .or_insert_with(Vec::new)
                        .push(entry);
                    Ok(final_result)
                },
            )
            .with_context(|| format!("Lỗi xử lý dữ liệu sheet {}", outflow_sheet_key))?;

        let unique_accounts = inflow_entries
            .keys()
            .chain(outflow_entries.keys())
            .cloned()
            .collect::<HashSet<_>>();

        let cashflow_by_account = unique_accounts
            .into_iter()
            .map(|account| {
                let inflows = inflow_entries.remove(&account).unwrap_or_default();
                let outflows = outflow_entries.remove(&account).unwrap_or_default();
                (account, (inflows, outflows))
            })
            .fold(
                HashMap::new(),
                |mut accounts, (account, (inflows, outflows))| {
                    accounts.insert(account, (inflows, outflows));
                    accounts
                },
            );

        let results = cashflow_by_account
            .into_iter()
            .map(|((cif, account), (inflows, outflows))| MoneyFlow {
                id: cif.clone().into(),
                subject_name: customer_infos
                    .get(&cif)
                    .cloned()
                    .unwrap_or_default()
                    .name
                    .into(),
                identification: customer_infos
                    .get(&cif)
                    .cloned()
                    .unwrap_or_default()
                    .id_number
                    .into(),
                account_number: account.clone().into(),
                bank_name: bank_accounts
                    .get(&(cif.clone(), account.clone()))
                    .and_then(|bank_info| bank_info.bank_name.clone()),
                bank_code: bank_accounts
                    .get(&(cif.clone(), account.clone()))
                    .and_then(|bank_info| bank_info.bank_code.clone()),
                total_converted_in: inflows
                    .iter()
                    .fold(0_f64, |acc, entry| {
                        acc + entry
                            .total_converted
                            .as_ref()
                            .and_then(|s| s.replace(",", "").parse::<f64>().ok())
                            .unwrap_or(0.0)
                    })
                    .to_string()
                    .into(),
                total_converted_out: outflows
                    .iter()
                    .fold(0_f64, |acc, entry| {
                        acc + entry
                            .total_converted
                            .as_ref()
                            .and_then(|s| s.replace(",", "").parse::<f64>().ok())
                            .unwrap_or(0.0)
                    })
                    .to_string()
                    .into(),
                total_transactions_in: inflows
                    .iter()
                    .fold(0_i64, |acc, entry| {
                        acc + entry
                            .total_transactions
                            .as_ref()
                            .and_then(|s| s.replace(",", "").parse::<i64>().ok())
                            .unwrap_or(0_i64)
                    })
                    .to_string()
                    .into(),
                total_transactions_out: outflows
                    .iter()
                    .fold(0_i64, |acc, entry| {
                        acc + entry
                            .total_transactions
                            .as_ref()
                            .and_then(|s| s.replace(",", "").parse::<i64>().ok())
                            .unwrap_or(0_i64)
                    })
                    .to_string()
                    .into(),
                inflows: inflows.into(),
                outflows: outflows.into(),
            })
            .collect::<Vec<_>>();

        Ok(results)
    }
}
