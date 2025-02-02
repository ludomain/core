use primitives::asset_price::{ChartValue, ChartPeriod};
use redis::{aio::Connection, AsyncCommands, RedisResult};
use storage::{database::DatabaseClient, models::{FiatRate, Chart}};
use std::{collections::HashMap, error::Error};

use storage::models::Price;

pub struct Client {
    conn: Connection,
    database: DatabaseClient,
    prefix: String,
}

impl Client {
    pub async fn new(redis_url: &str, database_url: &str) -> RedisResult<Self> {
        let client = redis::Client::open(redis_url)?;
        let conn = client.get_async_connection().await?;
        let database = DatabaseClient::new(database_url);

        Ok(Self {
            conn,
            database,
            prefix: "prices:".to_owned(),
        })
    }
    
    // db

    pub fn get_coin_id(&mut self, asset_id: &str) -> Result<String, Box<dyn Error>> {
        Ok(self.database.get_coin_id(asset_id)?)
    }

    pub async fn set_prices(&mut self, prices: Vec<Price>) -> Result<usize, Box<dyn Error>> {
        Ok(self.database.set_prices(prices)?)
    }

    pub fn get_prices(&mut self) -> Result<Vec<Price>, Box<dyn Error>> {
        Ok(self.database.get_prices()?)
    }

    pub async fn set_fiat_rates(&mut self, rates: Vec<FiatRate>) -> Result<usize, Box<dyn Error>> {
        Ok(self.database.set_fiat_rates(rates)?)
    } 

    pub fn get_fiat_rates(&mut self) -> Result<Vec<FiatRate>, Box<dyn Error>> {
        Ok(self.database.get_fiat_rates()?)
    }

    pub async fn set_charts(&mut self, charts: Vec<Chart>) -> Result<usize, Box<dyn Error>> {
        Ok(self.database.set_charts(charts)?)
    }

    pub fn get_charts_prices(&mut self, coin_id: &str, period: &ChartPeriod) -> Result<Vec<ChartValue>, Box<dyn Error>> {
        let prices = self.database
            .get_charts_prices(coin_id, period)?
            .into_iter().map(|x| 
                ChartValue{timestamp: (x.0.timestamp() as i32) , value: x.1}
            ).collect();
        Ok(prices)
    }

    // cache

    pub fn convert_asset_price_vec_to_map(coins: Vec<Price>) -> HashMap<String, Price> {
        coins.into_iter().map(|coin| (coin.asset_id.clone(), coin)).collect()
    }

    pub fn asset_key(&mut self, currency: &str, asset: String) -> String {
        format!("{}{}:{}", self.prefix, currency, asset)
    }

    pub async fn set_cache_prices(&mut self, currency: &str, prices: Vec<Price>) -> RedisResult<usize> {
        let serialized: Vec<(String, String)> = prices
        .iter()
        .map(|x| {
            (
                self.asset_key(currency, x.asset_id.clone()),
                serde_json::to_string(x).unwrap(),
            )
        })
        .collect();

        self.conn.mset(serialized.as_slice()).await?;

        Ok(serialized.len())
    }

    pub async fn get_cache_prices(&mut self, currency: &str, assets: Vec<&str>) -> RedisResult<Vec<Price>> {
        let keys: Vec<String> = assets.iter().map(|x| self.asset_key(currency, x.to_string())).collect();
        let result: Vec<Option<String>> = self
            .conn
            .mget(keys)
            .await?;

        let values: Vec<String> = result.into_iter().flatten().collect();
        let prices: Vec<Price> = values.iter().filter_map(|x| {
            serde_json::from_str(x).unwrap_or(None)
        }).collect();

        Ok(prices)
    }
}