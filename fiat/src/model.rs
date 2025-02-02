use std::collections::HashMap;
use primitives::{fiat_quote::FiatQuote, fiat_quote_request::FiatBuyRequest, fiat_provider::FiatProviderName};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use storage::models::FiatRate;

pub struct FiatRequestMap {
    pub crypto_currency: String,
    pub network: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiatRates {
    pub rates: Vec<FiatRate>
}

// mappings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiatMapping {
    pub symbol: String,
    pub network: Option<String>
}

pub type FiatMappingMap = HashMap<String, FiatMapping>;

#[async_trait]
pub trait FiatClient {
    fn name(&self) -> FiatProviderName;
    async fn get_quote(&self, request: FiatBuyRequest, request_map: FiatMapping) -> Result<FiatQuote, Box<dyn std::error::Error>>;
}