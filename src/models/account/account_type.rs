/// AccountLinkTypeEnum :
/// The account link types:
/// * 0 -  Unlinked. Account is not linked to another account.
/// * 1 -  Main. Account is a balance-holding account that is linked to a remote harvester account.
/// * 2 -  Remote. Account is a remote harvester account that is linked to a balance-holding account.
/// * 3 -  Remote_Unlinked. Account is a remote harvester eligible account that is unlinked.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AccountLinkTypeEnum {
    /// Unlinked.
    #[serde(rename = "0")]
    _0,

    /// Balance-holding account that is linked to a remote harvester account.
    #[serde(rename = "1")]
    _1,

    /// Remote harvester account that is linked to a balance-holding account.
    #[serde(rename = "2")]
    _2,

    /// Remote harvester eligible account that is unlinked.
    #[serde(rename = "3")]
    _3,
}
