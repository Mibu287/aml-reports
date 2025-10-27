use crate::{
    payload::{
        entities::{BeneficialOwners, Individual, IndividualLink, Organization, OrganizationLink},
        section2::Section2,
    },
    template::cell_value_from_key,
};

impl Section2 {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: std::io::Seek + std::io::Read,
    {
        Ok(Self {
            individuals: None,
            organizations: None,
            beneficial_owners: None,
            additional_info: Some(cell_value_from_key("Phần II: Thông tin bổ sung", workbook)?),
        })
    }
}
