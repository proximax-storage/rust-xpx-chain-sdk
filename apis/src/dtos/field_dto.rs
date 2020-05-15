#[derive(Serialize, Deserialize)]
pub struct FieldDto {
    #[serde(rename = "key")]
    key: String,
    #[serde(rename = "value")]
    value: String,
}