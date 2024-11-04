use crate::config::mysql_config;
use log::info;
use serde::{Deserialize, Serialize};

use sqlx::{mysql::MySqlPool, Row};

#[derive(Debug, Serialize, Deserialize)]
pub struct WhitelistUserData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    openid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    added_time: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    expiration_date: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    added_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    status: Option<i8>,
}

#[derive(Debug)]
pub struct MysqlClient {
    pub pool: MySqlPool,
}

impl MysqlClient {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let config = mysql_config::load_config()?;
        let url = config.to_url();
        Ok(MysqlClient {
            pool: MySqlPool::connect(&url).await?,
        })
    }

    pub async fn delete_whitelist_user(
        &self,
        data: WhitelistUserData,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let exec_cmd = format!(
            "DELETE FROM whitelist_user WHERE openid='{}';",
            data.openid.ok_or("openid required")?
        );
        info!("delete_whitelist_user exec_cmd:{}", exec_cmd);
        let result = sqlx::query(&exec_cmd).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn insert_whitelist_user(
        &self,
        data: WhitelistUserData,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let mut fields = Vec::new();
        let mut values = Vec::new();

        fields.push("openid");
        values.push(format!("'{}'", data.openid.ok_or("openid required")?));
        if let Some(name) = data.name {
            fields.push("name");
            values.push(format!("'{}'", name));
        }
        if let Some(added_time) = data.added_time {
            fields.push("added_time");
            values.push(format!("'{}'", added_time));
        }
        if let Some(expiration_date) = data.expiration_date {
            fields.push("expiration_date");
            values.push(format!("'{}'", expiration_date));
        }
        if let Some(added_by) = data.added_by {
            fields.push("added_by");
            values.push(format!("'{}'", added_by));
        }
        if let Some(status) = data.status {
            fields.push("status");
            values.push(format!("'{}'", status));
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

    pub async fn query_whitelist_user(
        &self,
        data: WhitelistUserData,
    ) -> Result<Vec<WhitelistUserData>, Box<dyn std::error::Error>> {
        info!("query_whitelist_user received data:{:?}", data);
        let query_cmd = match data.openid {
            Some(openid) => format!("SELECT * FROM whitelist_user WHERE openid='{}';", openid),
            None => String::from("SELECT * FROM whitelist_user;"),
        };
        let rows = sqlx::query(&query_cmd).fetch_all(&self.pool).await?;
        let res = rows
            .into_iter()
            .map(|row| WhitelistUserData {
                openid: row.get("openid"),
                name: row.get("name"),
                added_time: row.get("added_time"),
                expiration_date: row.get("expiration_date"),
                added_by: row.get("added_by"),
                status: row.get("status"),
            })
            .collect();
        Ok(res)
    }
}
