mod auth;
mod launch;
mod payload;

use auth::get_auth_code;
use launch::launch_web_automation_task;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = 9515;
    let (_, auth_key) = launch_web_automation_task(get_auth_code, port).await?;

    let form_payload = payload::form::Form {
        id: None,
        internal_number: "SOME-REPORT-003".to_string(),
        report_type: "M1".to_string(),
        creation_status: "DANG_NHAP_LIEU".to_string(),
        payload: payload::form::Payload {
            general_info: payload::general_info::GeneralInfo {
                report_date: "2024-06-01".to_string(),
                report_number: "RPT-001".to_string(),
                amendment_supplement: payload::general_info::AmendmentSupplement {
                    change_type: 0,
                    report_number: "".to_string(),
                    report_date: "".to_string(),
                },
                reporting_entity_name: "Entity Name".to_string(),
                reporting_entity_code: "ENT-001".to_string(),
                report_form: "Form A".to_string(),
            },
            section_1: payload::section1::Section1 {
                reporting_entity: payload::section1::ReportingEntity {
                    name: "Entity Name".to_string(),
                    code: "ENT-001".to_string(),
                    address: payload::section1::Address {
                        street_address: "123 Main St".to_string(),
                        phone: "555-1234".to_string(),
                        district: "Central".to_string(),
                        city_province: "Metropolis".to_string(),
                        country: "Freedonia".to_string(),
                    },
                    transaction_location: payload::section1::TransactionLocation {
                        street_address: "123 Main St".to_string(),
                        transaction_point_name: "Main Branch".to_string(),
                        phone: "555-1234".to_string(),
                        district: "Central".to_string(),
                        city_province: "Metropolis".to_string(),
                        country: "Freedonia".to_string(),
                    },
                    email: "entity@example.com".to_string(),
                },
                responsible_person: payload::section1::ResponsiblePerson {
                    full_name: "John Doe".to_string(),
                    work_phone: "555-1234".to_string(),
                    mobile_phone: "555-5678".to_string(),
                    position: "Manager".to_string(),
                },
                report_preparer: payload::section1::ReportPreparer {
                    full_name: "Jane Smith".to_string(),
                    work_phone: "123-456-7890".to_string(),
                    mobile_phone: "098-765-4321".to_string(),
                    department: "Compliance".to_string(),
                },
            },
            section_2: Default::default(),
            section_3: Default::default(),
            section_4: Default::default(),
            section_5: Default::default(),
            section_6: Default::default(),
        },
    };

    reqwest::Client::new()
        .post("https://amlstr.sbv.gov.vn/strcreator/api/str-creator/saveStrModel?tabNo=0")
        .bearer_auth(auth_key)
        .json(&form_payload)
        .send()
        .await?;

    Ok(())
}
