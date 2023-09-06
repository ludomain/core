extern crate rocket;
use primitives::Asset;
use rocket::serde::json::Json;
use self::client::AssetsClient;
use rocket::State;
use rocket::tokio::sync::Mutex;

pub mod client;

#[get("/assets/<asset_id>")]
pub async fn get_asset(
    asset_id: &str,
    client: &State<Mutex<AssetsClient>>,
) -> Json<Asset> {
    let asset = client.lock().await.get_asset(asset_id).unwrap();
    Json(asset)
}