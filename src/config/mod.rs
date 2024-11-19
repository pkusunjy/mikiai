use serde::Deserialize;
use std::{fs::File, io::Read};

// 集成配置
#[derive(Clone, Debug, Deserialize)]
pub struct IntegrationConfig {
    pub openai_url_root: String,
    pub openai_api_key: String,

    pub qwen_url_root: String,
    pub qwen_api_key: String,

    pub aliyun_oss_endpoint: String,
    pub aliyun_oss_access_key_id: String,
    pub aliyun_oss_access_key_secret: String,

    pub swagger_endpoint: String,

    pub mysql_ip: String,
    pub mysql_port: String,
    pub mysql_username: String,
    pub mysql_password: String,
    pub mysql_database: String,

    pub wechat_appid: String,
    pub wechat_mch_id: String,
    pub wechat_v3_key: String,
    pub wechat_secret: String,
    pub wechat_serial_no: String,
}

impl IntegrationConfig {
    pub fn gen_mysql_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.mysql_username,
            self.mysql_password,
            self.mysql_ip,
            self.mysql_port,
            self.mysql_database
        )
    }
    pub fn gen_utility_url_by_route(&self, route: &str) -> String {
        format!("http://{}{}", self.swagger_endpoint, route)
    }
}

pub fn load_integration_config() -> Result<IntegrationConfig, Box<dyn std::error::Error>> {
    let mut file = File::open("./conf/integration.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config = serde_yaml::from_str(&contents)?;
    Ok(config)
}
