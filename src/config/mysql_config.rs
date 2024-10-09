use serde::Deserialize;
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize)]
pub struct Config {
    ip: String,
    port: u32,
    username: String,
    password: String,
    database: String,
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.ip, self.port, self.database
        )
    }
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error + Send + Sync>> {
    let mut file = File::open("./conf/mysql.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config = serde_yaml::from_str(&contents)?;
    Ok(config)
}
