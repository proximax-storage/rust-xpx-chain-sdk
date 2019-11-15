#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicDto {
    #[serde(rename = "id")]
    pub id: Vec<i32>,
    #[serde(rename = "amount")]
    pub amount: Vec<i32>,
}

impl MosaicDto {
    pub fn new(id: Vec<i32>, amount: Vec<i32>) -> MosaicDto {
        MosaicDto {
            id,
            amount,
        }
    }
}


