use std::fmt;

use serde::Serialize;
pub mod guid;
pub mod sequential;
pub mod utils;

#[derive(Debug)]
pub struct AssetError {
    msg: String,
}
impl fmt::Display for AssetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

// impl From<AssetError> for ServiceError {
//     fn from(asset: AssetError) -> ServiceError {
//         ServiceError::BadRequest(asset.msg)
//     }
// }

// please keep these organized while editing
#[derive(PartialEq, Eq, Debug, Copy, Clone, Serialize)]
pub enum BrokerAsset {
    ADA,
    BTC,
    DOT,
    ETH,
    GRIN,
    USD,
}

impl BrokerAsset {
    pub fn from_string(asset: &str) -> Result<BrokerAsset, AssetError> {
        let upper = asset.to_uppercase();
        match upper.as_str() {
            "ADA" => Ok(BrokerAsset::ADA),
            "BTC" => Ok(BrokerAsset::BTC),
            "DOT" => Ok(BrokerAsset::DOT),
            "ETH" => Ok(BrokerAsset::ETH),
            "GRIN" => Ok(BrokerAsset::GRIN),
            "USD" => Ok(BrokerAsset::USD),
            _ => Err(AssetError {
                msg: format!("invalid asset: {}", asset),
            }),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            BrokerAsset::ADA => "ADA".to_string(),
            BrokerAsset::BTC => "BTC".to_string(),
            BrokerAsset::DOT => "DOT".to_string(),
            BrokerAsset::ETH => "ETH".to_string(),
            BrokerAsset::GRIN => "GRIN".to_string(),
            BrokerAsset::USD => "USD".to_string(),
        }
    }
}