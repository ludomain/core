use std::{error::Error, str::FromStr};
use async_trait::async_trait;
use primitives::{chain::Chain, name::{NameRecord, NameProvider}};
use serde::{Serialize, Deserialize};
use crate::client::NameClient;
use reqwest::Client;
use base64::{engine::general_purpose, Engine as _};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResolveDomain {
    pub s: String,
    pub result: String
}

pub struct SNSClient {
    url: String,
    client: Client,
}

impl SNSClient {
    pub fn new(
        url: String
    ) -> Self {
        let client = Client::new();
        Self {
            url,
            client,
        }
    }

    async fn resolve_hex_address(&self, name: &str, chain: &Chain, record: &str) -> Result<NameRecord, Box<dyn Error>> {
        let url = format!("{}/record/{}/{}", self.url, name, record);
        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<ResolveDomain>()
            .await?;
        
        let bytes = general_purpose::STANDARD.decode(response.result.as_bytes()).unwrap();
        let hex_string: String = bytes.iter().map(|byte| format!("{:02X}", byte)).collect::<String>();
        let address = ethaddr::Address::from_str(hex_string.as_str()).unwrap().to_string();

        Ok(NameRecord { name: name.to_string(), chain: *chain, address, provider: Self::provider() })
    }

    async fn resolve_sol_address(&self, name: &str, chain: &Chain) -> Result<NameRecord, Box<dyn Error>> {
        let url = format!("{}/resolve/{}", self.url, name);
        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<ResolveDomain>()
            .await?;

        if response.s != "ok" {
            return Err("error".to_string().into())
        }
        Ok(NameRecord { name: name.to_string(), chain: *chain, address: response.result, provider: Self::provider() })
    }
}

#[async_trait]
impl NameClient for SNSClient {
   
    fn provider() -> NameProvider {
        NameProvider::Sns
    }

    async fn resolve(&self, name: &str, chain: Chain) -> Result<NameRecord, Box<dyn Error>> {
        match chain {
            Chain::Solana => {
                return self.resolve_sol_address(name, &chain.clone()).await;
            },
            Chain::SmartChain => {
                return self.resolve_hex_address(name, &chain, "BSC").await;
            },
            _ => {
                return Err("error".to_string().into())
            }
        }
    }

    fn domains() -> Vec<&'static str> {
        vec![
            "sol"
        ]
    }

    fn chains() -> Vec<Chain> {
        vec![
            Chain::Solana,
            Chain::SmartChain,
        ]
    }
}