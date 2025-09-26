use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WalletImport {
    default_descriptor: String,
    key_details: Vec<KeyDetail>,
    signatures_needed: u32,
    number_of_xpubs: u32,
    policy_type: PolicyType,
    default_script_type: String,
    default_electrum_server_url: String,
    backend_server_base_url: String,
    default_network: String,
    is_using_public_server: bool,
    private_electrum_url: String,
    public_electrum_url: String,
    labels: HashMap<String, String>,
    btc_metric: String,
    fee_rate_color_map_values: Vec<(u32, String)>,
    fee_scale: ScaleOption,
    min_fee_scale: ScaleOption,
    fee_rate: u32,
    is_create_batch_tx: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeyDetail {
    master_fingerprint: String,
    xpub: String,
    derivation_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PolicyType {
    value: String,
    label: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScaleOption {
    value: String,
    label: String,
}
