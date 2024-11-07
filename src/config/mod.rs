use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

macro_rules! load_yaml_config {
    ($func_name: ident, $file_path: expr, $config_type: ty) => {
        pub fn $func_name() -> Result<$config_type, Box<dyn std::error::Error + Send + Sync>> {
            let mut file = File::open($file_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let config: $config_type = serde_yaml::from_str(&contents)?;
            Ok(config)
        }
    };
}

// MySql config
#[derive(Debug, Deserialize)]
pub struct MySqlConfig {
    ip: String,
    port: u32,
    username: String,
    password: String,
    database: String,
}

impl MySqlConfig {
    pub fn to_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.ip, self.port, self.database
        )
    }
}

load_yaml_config!(load_mysql_config, "./conf/mysql.yaml", MySqlConfig);

// WeChatPay config
#[derive(Debug, Deserialize, Serialize)]
pub struct WeChatPayConfig {
    appid: String,
    mch_id: String,
    private_key: String,
    serial_no: String,
    v3_key: String,
    notify_url: String,
    base_url: String,
}

#[rustfmt::skip]
load_yaml_config!(load_wechatpay_config, "./conf/wechatpay.yaml", WeChatPayConfig);
