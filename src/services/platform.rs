use crate::config::IntegrationConfig;
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::Row;

// 白名单用户信息
#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct WhitelistUserData {
    pub openid: Option<String>,
    pub name: Option<String>,
    pub added_time: Option<u64>,
    pub expiration_date: Option<u64>,
    pub added_by: Option<String>,
    pub status: Option<i8>,
}

impl WhitelistUserData {
    pub fn new_with_openid(openid: String) -> Self {
        WhitelistUserData {
            openid: Some(openid),
            name: None,
            added_time: None,
            expiration_date: None,
            added_by: None,
            status: None,
        }
    }
    pub fn gen_insert_fields_and_values(&self) -> (String, String) {
        let mut fields = vec![];
        let mut values = vec![];
        if let Some(openid) = &self.openid {
            fields.push("openid");
            values.push(openid.clone());
        }
        if let Some(name) = &self.name {
            fields.push("name");
            values.push(name.clone());
        }
        if let Some(added_time) = &self.added_time {
            fields.push("added_time");
            values.push(format!("{}", added_time));
        }
        if let Some(expiration_date) = &self.expiration_date {
            fields.push("expiration_date");
            values.push(format!("{}", expiration_date));
        }
        if let Some(added_by) = &self.added_by {
            fields.push("added_by");
            values.push(added_by.clone());
        }
        if let Some(status) = &self.status {
            fields.push("status");
            values.push(format!("{}", status));
        }
        let fields_str = format!("({})", fields.join(", "));
        let values_str = format!(
            "({})",
            values
                .iter()
                .map(|v| v.as_str())
                .collect::<Vec<&str>>()
                .join(", ")
        );
        return (fields_str, values_str);
    }
    pub fn gen_update_cmd(&self) -> String {
        let mut fields = vec![];
        if let Some(name) = &self.name {
            fields.push(format!("name = '{}'", name.clone()));
        }
        if let Some(added_time) = &self.added_time {
            fields.push(format!("added_time = '{}'", added_time));
        }
        if let Some(expiration_date) = &self.expiration_date {
            fields.push(format!("expiration_date = '{}'", expiration_date));
        }
        if let Some(added_by) = &self.added_by {
            fields.push(format!("added_by = '{}'", added_by.clone()));
        }
        if let Some(status) = &self.status {
            fields.push(format!("status = '{}'", status));
        }
        return fields.join(", ");
    }
}

#[derive(Debug)]
pub struct DataPlatformService {
    pub pool: sqlx::mysql::MySqlPool,
}

impl DataPlatformService {
    pub async fn new(
        conf: &IntegrationConfig,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let url = conf.gen_mysql_url();
        Ok(DataPlatformService {
            pool: sqlx::mysql::MySqlPool::connect(&url).await?,
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
        info!("[delete_whitelist_user] exec_cmd:{}", exec_cmd);
        let result = sqlx::query(&exec_cmd).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn insert_whitelist_user(
        &self,
        data: WhitelistUserData,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let (fields, values) = data.gen_insert_fields_and_values();
        let exec_cmd = format!("INSERT INTO whitelist_user {} VALUES {}", fields, values);
        info!("[insert_whitelist_user] exec_cmd:{}", exec_cmd);
        let result = sqlx::query(&exec_cmd).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn query_whitelist_user(
        &self,
        data: WhitelistUserData,
    ) -> Result<Vec<WhitelistUserData>, Box<dyn std::error::Error>> {
        let exec_cmd = match &data.openid {
            Some(openid) => format!(
                "SELECT * FROM whitelist_user WHERE openid='{}' AND expiration_date > UNIX_TIMESTAMP(NOW());",
                openid
            ),
            None => String::from("SELECT * FROM whitelist_user WHERE expiration_date > UNIX_TIMESTAMP(NOW());"),
        };
        info!("[query_whitelist_user] exec_cmd:{}", exec_cmd);
        let rows = sqlx::query(&exec_cmd).fetch_all(&self.pool).await?;
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

    pub async fn update_whitelist_user(
        &self,
        data: WhitelistUserData,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let exec_cmd = format!(
            "UPDATE whitelist_user SET {} WHERE openid = '{}'",
            data.gen_update_cmd(),
            data.openid.ok_or("openid required")?
        );
        info!("[update_whitelist_user] exec_cmd: {}", exec_cmd);
        let result = sqlx::query(&exec_cmd).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }
}
