#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionBodyDto {
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
}

impl TransactionBodyDto {
    pub fn new(max_fee: Vec<i32>, deadline: Vec<i32>) -> TransactionBodyDto {
        TransactionBodyDto {
            max_fee,
            deadline,
        }
    }
}


