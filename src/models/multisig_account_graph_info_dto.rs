#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MultisigAccountGraphInfoDto {
    /// The level of the multisig account.
    #[serde(rename = "level")]
    pub level: i32,
    /// The array of multisig accounts for this level.
    #[serde(rename = "multisigEntries")]
    pub multisig_entries: Vec<crate::models::MultisigAccountInfoDto>,
}

impl MultisigAccountGraphInfoDto {
    pub fn new(level: i32, multisig_entries: Vec<crate::models::MultisigAccountInfoDto>) -> MultisigAccountGraphInfoDto {
        MultisigAccountGraphInfoDto {
            level,
            multisig_entries,
        }
    }
}


