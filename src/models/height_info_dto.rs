#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HeightInfoDto {
    #[serde(rename = "height")]
    pub height: Vec<i32>,
}

impl HeightInfoDto {
    pub fn new(height: Vec<i32>) -> HeightInfoDto {
        HeightInfoDto {
            height,
        }
    }
}


