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
        internal_number: "SOME-REPORT-010".to_string(),
        report_type: "M1".to_string(),
        creation_status: "DANG_NHAP_LIEU".to_string(),
        payload: payload::form::Payload {
            general_info: payload::info::GeneralInfo {
                report_date: "2025-10-24".to_string(),
                report_number: "RPT-001".to_string(),
                amendment_supplement: payload::info::AmendmentSupplement {
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
            section_2: payload::section2::Section2 {
                individuals: Some(vec![
                    payload::entities::Individual {
                        id: Some(1761286439631),
                        existing_customer: Some("1".to_string()),
                        full_name: Some("Nguyễn Văn C".to_string()),
                        date_of_birth: Some("1966-10-04T16:00:00.000Z".to_string()),
                        age: Some("5".to_string()),
                        gender: Some("male".to_string()),
                        nationality: Some("VN".to_string()),
                        occupation: Some(payload::entities::Occupation {
                            occupation_code: Some("2".to_string()),
                            description: Some("Học sinh/sinh viên".to_string()),
                            content: None,
                        }),
                        position: Some("Chuyên viên".to_string()),
                        permanent_address: Some(payload::entities::AddrSimple {
                            street_address: Some("Hà Nội".to_string()),
                            district: Some("Hoàn Kiếm".to_string()),
                            city_province: Some("Hà Nội".to_string()),
                            country: Some("VN".to_string()),
                            phone: None,
                        }),
                        current_address: Some(payload::entities::AddrSimple {
                            street_address: Some("Lý Thường Kiệt".to_string()),
                            district: Some("Hoàn Kiếm".to_string()),
                            city_province: Some("Hà Nội".to_string()),
                            country: Some("VN".to_string()),
                            phone: None,
                        }),
                        identifications: Some(vec![payload::entities::Identification {
                            id_type: Some("101".to_string()),
                            id_number: Some("036100018836".to_string()),
                            issue_date: Some("2025-10-16T17:00:00.000Z".to_string()),
                            expiry_date: Some("2035-09-30T17:00:00.000Z".to_string()),
                            issuing_authority: Some("Hà Nội".to_string()),
                            place_of_issue: Some("Hà Nội".to_string()),
                        }]),
                        phone_number: Some("0987654321".to_string()),
                        education_level: Some("Đại học".to_string()),
                        email: Some("abc@gmail.com".to_string()),
                        accounts: Some(vec![payload::entities::Account {
                            account_number: Some("0440234837421".to_string()),
                            bank: Some(payload::entities::Bank {
                                bank_code: Some("01203001".to_string()),
                                bank_name: Some("Ngân hàng TMCP Ngoại thương Việt Nam".to_string()),
                            }),
                            currency_type: Some("VND".to_string()),
                            account_type: Some("CURRE".to_string()),
                            open_date: Some("2025-09-30T17:00:00.000Z".to_string()),
                            status: Some("ACTIV".to_string()),
                            authorized_persons: Some(vec![]),
                        }]),
                    },
                    payload::entities::Individual {
                        id: Some(1761286873020),
                        existing_customer: Some("1".to_string()),
                        full_name: Some("Nguyễn Văn D".to_string()),
                        date_of_birth: Some("2025-09-30T17:00:00.000Z".to_string()),
                        age: Some("5".to_string()),
                        gender: Some("male".to_string()),
                        nationality: Some("VN".to_string()),
                        occupation: Some(payload::entities::Occupation {
                            occupation_code: Some("9".to_string()),
                            description: Some("Lao động tự do".to_string()),
                            content: None,
                        }),
                        position: Some("jkfladjfl".to_string()),
                        permanent_address: Some(payload::entities::AddrSimple {
                            street_address: Some("jfdasklf".to_string()),
                            district: Some("ádjfl;kadf".to_string()),
                            city_province: Some("fdasjdfkl".to_string()),
                            country: Some("VN".to_string()),
                            phone: None,
                        }),
                        current_address: Some(payload::entities::AddrSimple {
                            street_address: Some("123412".to_string()),
                            district: Some("12343124".to_string()),
                            city_province: Some("hà Nội".to_string()),
                            country: Some("VN".to_string()),
                            phone: None,
                        }),
                        identifications: Some(vec![payload::entities::Identification {
                            id_type: Some("103".to_string()),
                            id_number: Some("0791993012333".to_string()),
                            issue_date: Some("2025-09-30T17:00:00.000Z".to_string()),
                            expiry_date: Some("2025-10-04T17:00:00.000Z".to_string()),
                            issuing_authority: Some("Hà Nội".to_string()),
                            place_of_issue: Some("Hà Nội".to_string()),
                        }]),
                        phone_number: Some("1324798013247".to_string()),
                        education_level: Some("Đại học".to_string()),
                        email: Some("abc@def.com".to_string()),
                        accounts: Some(vec![payload::entities::Account {
                            account_number: Some("123412343124".to_string()),
                            bank: Some(payload::entities::Bank {
                                bank_code: Some("01203001".to_string()),
                                bank_name: Some("Ngân hàng TMCP Ngoại thương Việt Nam".to_string()),
                            }),
                            currency_type: Some("VND".to_string()),
                            account_type: Some("CURRE".to_string()),
                            open_date: Some("2025-09-30T17:00:00.000Z".to_string()),
                            status: Some("ACTIV".to_string()),
                            authorized_persons: Some(vec![]),
                        }]),
                    },
                ]),
                organizations: Some(vec![payload::entities::Organization {
                    id: Some(1761286601844),
                    existing_customer: Some("1".to_string()),
                    name: Some("Công ty X".to_string()),
                    foreign_name: Some("X Ltd".to_string()),
                    short_name: Some("ABC".to_string()),
                    organization_type: Some(payload::entities::CodeDesc {
                        type_code: Some("3".to_string()),
                        description: Some("Công ty cổ phần".to_string()),
                    }),
                    address: Some(payload::entities::AddrSimple {
                        street_address: Some("12342".to_string()),
                        district: Some("13424321".to_string()),
                        city_province: Some("Hà Nội".to_string()),
                        country: Some("VN".to_string()),
                        phone: None,
                    }),
                    establishment_license: Some(payload::entities::License {
                        license_number: Some("13243124321".to_string()),
                        issue_date: Some("2025-09-30T17:00:00.000Z".to_string()),
                        issue_place: Some("Hà Nội".to_string()),
                    }),
                    enterprise_code: Some(payload::entities::EnterpriseCode {
                        code: Some("0109577331".to_string()),
                        issue_date: Some("2025-09-30T17:00:00.000Z".to_string()),
                        issue_place: Some("VN".to_string()),
                    }),
                    business_sector: Some("Hoá chất".to_string()),
                    phone_number: Some("0987654321".to_string()),
                    website: Some("https://abc.com".to_string()),
                    accounts: Some(vec![payload::entities::Account {
                        account_number: Some("12347190243".to_string()),
                        bank: Some(payload::entities::Bank {
                            bank_code: Some("01203001".to_string()),
                            bank_name: Some("Ngân hàng TMCP Ngoại thương Việt Nam".to_string()),
                        }),
                        currency_type: Some("VND".to_string()),
                        account_type: Some("CURRE".to_string()),
                        open_date: Some("2025-09-30T17:00:00.000Z".to_string()),
                        status: Some("ACTIV".to_string()),
                        authorized_persons: Some(vec![]),
                    }]),
                    representatives: Some(vec![payload::entities::Representative {
                        id: Some(1761286984066),
                        full_name: Some("ABC".to_string()),
                        date_of_birth: Some("1955-10-20T17:00:00.000Z".to_string()),
                        occupation: Some(payload::entities::Occupation {
                            occupation_code: Some("999".to_string()),
                            description: Some("Khác (tự nhập)".to_string()),
                            content: Some("dfasfdas".to_string()),
                        }),
                        position: Some("adsffda".to_string()),
                        permanent_address: Some(payload::entities::AddrSimple {
                            street_address: Some("sdafdasfd".to_string()),
                            district: Some("dầd".to_string()),
                            city_province: Some("ádfdasfd".to_string()),
                            country: Some("VN".to_string()),
                            phone: None,
                        }),
                        current_address: Some(payload::entities::AddrSimple {
                            street_address: Some("313424312".to_string()),
                            district: Some("ádfadf".to_string()),
                            city_province: Some("Hà Nội".to_string()),
                            country: Some("VN".to_string()),
                            phone: None,
                        }),
                        phone_number: Some("21341324".to_string()),
                        nationality: Some("VN".to_string()),
                        identifications: Some(vec![payload::entities::Identification {
                            id_type: Some("103".to_string()),
                            id_number: Some("0791993012333".to_string()),
                            issue_date: Some("2025-09-30T17:00:00.000Z".to_string()),
                            expiry_date: Some("2025-09-30T17:00:00.000Z".to_string()),
                            issuing_authority: Some("Hà Nội".to_string()),
                            place_of_issue: Some("Hà Nội".to_string()),
                        }]),
                    }]),
                }]),
                beneficial_owners: Some(payload::entities::BeneficialOwners {
                    other_owners: Some(vec![payload::entities::Individual {
                        id: Some(1761286765722),
                        existing_customer: Default::default(),
                        full_name: Some("12341243".to_string()),
                        date_of_birth: Some("2025-10-01T17:00:00.000Z".to_string()),
                        age: Some("5".to_string()),
                        gender: Some("male".to_string()),
                        nationality: Some("VN".to_string()),
                        occupation: Some(payload::entities::Occupation {
                            occupation_code: Some("999".to_string()),
                            description: Some("Khác (tự nhập)".to_string()),
                            content: Some("adfasdfdsa".to_string()),
                        }),
                        position: Some("fdasfqewfreqw".to_string()),
                        permanent_address: Some(payload::entities::AddrSimple {
                            street_address: Some("13244132".to_string()),
                            district: Some("12344312".to_string()),
                            city_province: Some("Hà Nội".to_string()),
                            country: Some("VN".to_string()),
                            phone: None,
                        }),
                        current_address: Some(payload::entities::AddrSimple {
                            street_address: Some("1234132".to_string()),
                            district: Some("Hoàn Kiếm".to_string()),
                            city_province: Some("Hoàn Kiếm".to_string()),
                            country: Some("VN".to_string()),
                            phone: None,
                        }),
                        identifications: Some(vec![payload::entities::Identification {
                            id_type: Some("103".to_string()),
                            id_number: Some("0791993012333".to_string()),
                            issue_date: Some("2025-09-30T17:00:00.000Z".to_string()),
                            expiry_date: Some("2025-09-30T17:00:00.000Z".to_string()),
                            issuing_authority: Some("Hà Nội".to_string()),
                            place_of_issue: Some("Hà Nội".to_string()),
                        }]),
                        phone_number: Some("0987654321".to_string()),
                        education_level: Default::default(),
                        email: Default::default(),
                        accounts: Default::default(),
                    }]),
                    individual_links: Some(vec![
                        payload::entities::IndividualLink {
                            name: Some("Nguyễn Văn C".to_string()),
                            identification_number: Some("036100018836".to_string()),
                            id: Some(1761286439631),
                            is_principal: Some(true),
                            benefit_group: Some(payload::entities::GroupBenefits {
                                main_group: Some(vec![payload::entities::PersonRef {
                                    full_name: Some("Nguyễn Văn D".to_string()),
                                    identification_number: Some("0791993012333".to_string()),
                                    id: Some(1761286873020),
                                }]),
                                other_group: Some(vec![payload::entities::PersonRef {
                                    full_name: Some("12341243".to_string()),
                                    identification_number: Some("0791993012333".to_string()),
                                    id: Some(1761286765722),
                                }]),
                            }),
                        },
                        payload::entities::IndividualLink {
                            name: Some("Nguyễn Văn D".to_string()),
                            identification_number: Some("0791993012333".to_string()),
                            id: Some(1761286873020),
                            is_principal: Some(true),
                            benefit_group: Some(payload::entities::GroupBenefits {
                                main_group: Some(vec![payload::entities::PersonRef {
                                    full_name: Some("Nguyễn Văn C".to_string()),
                                    identification_number: Some("036100018836".to_string()),
                                    id: Some(1761286439631),
                                }]),
                                other_group: Some(vec![payload::entities::PersonRef {
                                    full_name: Some("12341243".to_string()),
                                    identification_number: Some("0791993012333".to_string()),
                                    id: Some(1761286765722),
                                }]),
                            }),
                        },
                    ]),
                    organization_links: Some(vec![payload::entities::OrganizationLink {
                        id: Some(1761286601844),
                        name: Some("Công ty X".to_string()),
                        identification_number: Some("0109577331".to_string()),
                        benefit_group: Some(payload::entities::GroupBenefits {
                            main_group: Some(vec![
                                payload::entities::PersonRef {
                                    full_name: Some("Nguyễn Văn C".to_string()),
                                    identification_number: Some("036100018836".to_string()),
                                    id: Some(1761286439631),
                                },
                                payload::entities::PersonRef {
                                    full_name: Some("Nguyễn Văn D".to_string()),
                                    identification_number: Some("0791993012333".to_string()),
                                    id: Some(1761286873020),
                                },
                            ]),
                            other_group: Some(vec![payload::entities::PersonRef {
                                full_name: Some("12341243".to_string()),
                                identification_number: Some("0791993012333".to_string()),
                                id: Some(1761286765722),
                            }]),
                        }),
                    }]),
                }),
                additional_info: Some("Không có thông tin khác".to_string()),
            },
            section_3: payload::section3::Section3 {
                related_individuals: Some(vec![
                    payload::entities::Individual {
                        id: Some(1761296288535),
                        existing_customer: Default::default(),
                        full_name: Some("aslkdfjdklas".to_string()),
                        date_of_birth: Some("1975-10-07T17:00:00.000Z".to_string()),
                        age: Some("5".to_string()),
                        gender: Some("male".to_string()),
                        nationality: Some("VN".to_string()),
                        occupation: Some(payload::entities::Occupation {
                            occupation_code: Some("999".to_string()),
                            description: Some("Khác (tự nhập)".to_string()),
                            content: Some("abc".to_string()),
                        }),
                        position: Some("akldsfjl;kaf".to_string()),
                        permanent_address: Some(payload::entities::AddrSimple {
                            street_address: Some("jfasldkfjl;kads".to_string()),
                            district: Some("reqjwrlkjeq;lwk".to_string()),
                            city_province: Some("Hà Nội".to_string()),
                            country: Some("VN".to_string()),
                            phone: None,
                        }),
                        current_address: Some(payload::entities::AddrSimple {
                            street_address: Some("12433124".to_string()),
                            district: Some("Hoàn Kiếm".to_string()),
                            city_province: Some("Hà Nội".to_string()),
                            country: Some("VN".to_string()),
                            phone: None,
                        }),
                        identifications: Some(vec![payload::entities::Identification {
                            id_type: Some("103".to_string()),
                            id_number: Some("43124321".to_string()),
                            issue_date: Some("2025-10-08T17:00:00.000Z".to_string()),
                            expiry_date: Some("2025-10-30T17:00:00.000Z".to_string()),
                            issuing_authority: Some("jflkdas".to_string()),
                            place_of_issue: Some("qewrrqg".to_string()),
                        }]),
                        phone_number: Some("3127498".to_string()),
                        education_level: Default::default(),
                        email: Default::default(),
                        accounts: Some(vec![payload::entities::Account {
                            account_number: Some("1344123".to_string()),
                            bank: Some(payload::entities::Bank {
                                bank_code: Some("01203001".to_string()),
                                bank_name: Some("Ngân hàng TMCP Ngoại thương Việt Nam".to_string()),
                            }),
                            currency_type: Some("VND".to_string()),
                            account_type: Some("CURRE".to_string()),
                            open_date: Some("2025-10-23T17:00:00.000Z".to_string()),
                            status: Some("ACTIV".to_string()),
                            authorized_persons: Default::default(),
                        }]),
                    },
                    payload::entities::Individual {
                        id: Some(1761296493083),
                        existing_customer: Default::default(),
                        full_name: Some("fjdfleqw".to_string()),
                        date_of_birth: Some("1949-10-18T16:00:00.000Z".to_string()),
                        age: Some("5".to_string()),
                        gender: Some("male".to_string()),
                        nationality: Some("VN".to_string()),
                        occupation: Some(payload::entities::Occupation {
                            occupation_code: Some("3".to_string()),
                            description: Some("Giáo viên".to_string()),
                            content: Default::default(),
                        }),
                        position: Some("vádfdasf".to_string()),
                        permanent_address: Some(payload::entities::AddrSimple {
                            street_address: Some("43124321".to_string()),
                            district: Some("ádffdas".to_string()),
                            city_province: Some("fdasfdas".to_string()),
                            country: Some("VN".to_string()),
                            phone: None,
                        }),
                        current_address: Some(payload::entities::AddrSimple {
                            street_address: Some("143234321".to_string()),
                            district: Some("dfsafdas".to_string()),
                            city_province: Some("fadsfdas".to_string()),
                            country: Some("VN".to_string()),
                            phone: None,
                        }),
                        identifications: Some(vec![
                            payload::entities::Identification {
                                id_type: Some("103".to_string()),
                                id_number: Some("1234312".to_string()),
                                issue_date: Some("2025-10-23T17:00:00.000Z".to_string()),
                                expiry_date: Some("2025-11-07T17:00:00.000Z".to_string()),
                                issuing_authority: Some("dsaf".to_string()),
                                place_of_issue: Some("rểqwr".to_string()),
                            },
                            payload::entities::Identification {
                                id_type: Some("103".to_string()),
                                id_number: Some("31234312".to_string()),
                                issue_date: Some("2025-10-05T17:00:00.000Z".to_string()),
                                expiry_date: Some("2025-11-08T17:00:00.000Z".to_string()),
                                issuing_authority: Some("412fdas".to_string()),
                                place_of_issue: Some("43124312".to_string()),
                            },
                        ]),
                        phone_number: Some("43124312".to_string()),
                        education_level: Default::default(),
                        email: Default::default(),
                        accounts: Some(vec![
                            payload::entities::Account {
                                account_number: Some("43124312".to_string()),
                                bank: Some(payload::entities::Bank {
                                    bank_code: Some("01203001".to_string()),
                                    bank_name: Some("Ngân hàng TMCP Ngoại thương Việt Nam".to_string()),
                                }),
                                currency_type: Some("VND".to_string()),
                                account_type: Some("INVES".to_string()),
                                open_date: Some("2025-10-14T17:00:00.000Z".to_string()),
                                status: Some("BLOCK".to_string()),
                                authorized_persons: Default::default(),
                            },
                            payload::entities::Account {
                                account_number: Some("43124312".to_string()),
                                bank: Some(payload::entities::Bank {
                                    bank_code: Some("01667001".to_string()),
                                    bank_name: Some("Ngân hàng Bangkok Đại chúng TNHH - Chi nhánh Hà Nội".to_string()),
                                }),
                                currency_type: Some("USD".to_string()),
                                account_type: Some("SECUR".to_string()),
                                open_date: Some("2025-10-23T17:00:00.000Z".to_string()),
                                status: Some("HOLDS".to_string()),
                                authorized_persons: Default::default(),
                            },
                        ]),
                    },
                ]),
                related_organizations: Some(vec![payload::entities::Organization {
                    id: Some(1761296382254),
                    existing_customer: Default::default(),
                    name: Some("1342fqdf".to_string()),
                    foreign_name: Some("r1324312".to_string()),
                    short_name: Some("fwqfeqwr".to_string()),
                    organization_type: Default::default(),
                    address: Some(payload::entities::AddrSimple {
                        street_address: Some("qưereqwr1234".to_string()),
                        district: Some("1324312".to_string()),
                        city_province: Some("4124312".to_string()),
                        country: Some("VN".to_string()),
                        phone: None,
                    }),
                    establishment_license: Some(payload::entities::License {
                        license_number: Some("1243132".to_string()),
                        issue_date: Some("2025-10-16T17:00:00.000Z".to_string()),
                        issue_place: Some("Việt Nam".to_string()),
                    }),
                    enterprise_code: Some(payload::entities::EnterpriseCode {
                        code: Some("0314537155".to_string()),
                        issue_date: Some("2025-09-30T17:00:00.000Z".to_string()),
                        issue_place: Some("VN".to_string()),
                    }),
                    business_sector: Some("re21r321423".to_string()),
                    phone_number: Some("reqreqwr".to_string()),
                    website: Some("https://abc.def".to_string()),
                    accounts: Some(vec![
                        payload::entities::Account {
                            account_number: Some("fewrewq".to_string()),
                            bank: Some(payload::entities::Bank {
                                bank_code: Some("01201001".to_string()),
                                bank_name: Some("Ngân hàng TMCP Công Thương Việt Nam ".to_string()),
                            }),
                            currency_type: Some("VND".to_string()),
                            account_type: Some("CURRE".to_string()),
                            open_date: Some("2025-10-06T17:00:00.000Z".to_string()),
                            status: Some("ACTIV".to_string()),
                            authorized_persons: Default::default(),
                        },
                        payload::entities::Account {
                            account_number: Some("32143124".to_string()),
                            bank: Some(payload::entities::Bank {
                                bank_code: Some("01201001".to_string()),
                                bank_name: Some("Ngân hàng TMCP Công Thương Việt Nam ".to_string()),
                            }),
                            currency_type: Some("USD".to_string()),
                            account_type: Some("SAVIN".to_string()),
                            open_date: Some("2025-10-07T17:00:00.000Z".to_string()),
                            status: Some("BLOCK".to_string()),
                            authorized_persons: Default::default(),
                        },
                    ]),
                    representatives: Default::default(),
                }]),
                additional_info: Some("Không có thông tin bổ sung".to_string()),
            },
            section_4: Default::default(),
            section_5: Default::default(),
            section_6: Default::default(),
        },
    };

    let resp = reqwest::Client::new()
        .post("https://amlstr.sbv.gov.vn/strcreator/api/str-creator/saveStrModel?tabNo=0")
        .bearer_auth(auth_key)
        .json(&form_payload)
        .send()
        .await?;

    println!("Response: {:?}", resp.text().await?);

    Ok(())
}
