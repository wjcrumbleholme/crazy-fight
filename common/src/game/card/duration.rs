use serde::Deserialize;


#[derive(Deserialize, Debug, Clone)]
pub struct Duration {
    period: String,
    amount: Option<i32>,
}

impl Duration {
    pub fn new(period: String, amount: Option<i32>) -> Self {
        Self {
            period: period,
            amount: amount
        }
    }
}