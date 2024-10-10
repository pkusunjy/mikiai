use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    appid: String,
    mch_id: String,
    private_key: String,
    serial_no: String,
    v3_key: String,
    notify_url: String,
    base_url: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error + Send + Sync>> {
    let mut file = File::open("./conf/wechatpay.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config = serde_yaml::from_str(&contents)?;
    Ok(config)
}
