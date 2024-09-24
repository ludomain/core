use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::PlatformStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[typeshare(swift = "Codable")]
#[serde(rename_all = "camelCase")]
pub struct ConfigResponse {
    pub app: ConfigApp,
    pub releases: Vec<Release>,
    //TODO: Remove later
    pub versions: ConfigVersions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[typeshare(swift = "Codable")]
pub struct ConfigApp {
    pub ios: ConfigIOSApp,
    pub android: ConfigAndroidApp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[typeshare(swift = "Codable")]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub version: String,
    pub store: PlatformStore,
    pub upgrade_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[typeshare(swift = "Codable")]
pub struct ConfigIOSApp {
    pub version: ConfigAppVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[typeshare(swift = "Codable")]
pub struct ConfigAndroidApp {
    pub version: ConfigAppVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[typeshare(swift = "Codable")]
pub struct ConfigAppVersion {
    pub production: String,
    pub beta: String,
    pub alpha: String,
}

#[typeshare(swift = "Codable")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigVersions {
    pub fiat_assets: i32,
    pub swap_assets: i32,
}
