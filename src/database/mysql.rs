use crate::config::mysql_config;
use log::info;
use serde::{Deserialize, Serialize};

use sqlx::mysql::MySqlPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct WhitelistUserData {
    openid: Option<String>,
    name: Option<String>,
    added_time: Option<u64>,
    expiration_date: Option<u64>,
    added_by: Option<String>,
    status: Option<i8>,
}

pub struct MysqlClient {
    pool: MySqlPool,
}

impl MysqlClient {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = mysql_config::load_config()?;
        let url = config.to_string();
        Ok(MysqlClient {
            pool: MySqlPool::connect(&url).await?,
        })
    }

    pub async fn insert_whitelist_user(
        &self,
        data: WhitelistUserData,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let mut fields = Vec::new();
        let mut values = Vec::new();

        fields.push("openid");
        values.push(data.openid.ok_or("openid required")?);
        if let Some(name) = data.name {
            fields.push("name");
            values.push(name);
        }
        if let Some(added_time) = data.added_time {
            fields.push("added_time");
            values.push(added_time.to_string());
        }
        if let Some(expiration_date) = data.expiration_date {
            fields.push("expiration_date");
            values.push(expiration_date.to_string());
        }
        if let Some(added_by) = data.added_by {
            fields.push("added_by");
            values.push(added_by.to_string());
        }
        if let Some(status) = data.status {
            fields.push("status");
            values.push(status.to_string());
        }
        let exec_cmd = format!(
            "INSERT INTO whitelist_user {} VALUES {};",
            format!("({})", fields.join(",")),
            format!("({})", values.join(","))
        );

        info!("insert_whitelist_user exec_cmd: {}", exec_cmd);

        let result = sqlx::query(&exec_cmd).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn update_whitelist_user(
        &self,
        data: WhitelistUserData,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let mut kv = Vec::new();
        if let Some(name) = data.name {
            kv.push(format!("name = '{}'", name));
        }
        if let Some(added_time) = data.added_time {
            kv.push(format!("added_time = '{}'", added_time));
        }
        if let Some(expiration_date) = data.expiration_date {
            kv.push(format!("expiration_date = '{}'", expiration_date));
        }
        if let Some(added_by) = data.added_by {
            kv.push(format!("added_by = '{}'", added_by));
        }
        if let Some(status) = data.status {
            kv.push(format!("status = '{}'", status));
        }
        let exec_cmd = format!(
            "UPDATE whitelist_user SET {} WHERE openid = '{}'",
            kv.join(","),
            data.openid.ok_or("openid required")?
        );
        info!("update_whitelist_user exec_cmd: {}", exec_cmd);
        let result = sqlx::query(&exec_cmd).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }
}
