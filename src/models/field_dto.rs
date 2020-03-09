#[derive(Serialize, Deserialize)]
pub struct FieldDto {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}