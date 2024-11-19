use crate::config::IntegrationConfig;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CodeToSessionRequest {
    code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CodeToSessionResponse {
    errorcode: i32,
    errmsg: String,
    openid: String,
    #[serde(rename = "sessionKey")]
    session_key: String,
    unionid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenService {
    pub aliyun_oss_endpoint: String,
    pub aliyun_oss_access_key_id: String,
    pub aliyun_oss_access_key_secret: String,
    pub wechat_appid: String,
    pub wechat_secret: String,
}

impl TokenService {
    pub fn new(conf: &IntegrationConfig) -> Self {
        TokenService {
            aliyun_oss_endpoint: conf.aliyun_oss_endpoint.clone(),
            aliyun_oss_access_key_id: conf.aliyun_oss_access_key_id.clone(),
            aliyun_oss_access_key_secret: conf.aliyun_oss_access_key_secret.clone(),
            wechat_appid: conf.wechat_appid.clone(),
            wechat_secret: conf.wechat_secret.clone(),
        }
    }

    pub async fn jscode_to_session(
        &self,
        req: CodeToSessionRequest,
    ) -> Result<CodeToSessionResponse, Box<dyn std::error::Error>> {
        let code = req.code.ok_or("code required")?;
        let url = format!("https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code", self.wechat_appid, self.wechat_secret, code);
        let mut resp = awc::Client::default().get(&url).send().await?;
        let resp_body = String::from_utf8(resp.body().await?.to_vec())?;
        info!(
            "[jscode_to_session] url {} response body: {}",
            url, resp_body
        );
        let resp: CodeToSessionResponse = serde_json::from_str(&resp_body)?;
        Ok(resp)
    }
}
