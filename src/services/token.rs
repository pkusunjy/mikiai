use crate::config::IntegrationConfig;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CodeToSessionRequest {
    code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CodeToSessionResponse {
    errcode: Option<i32>,
    errmsg: Option<String>,
    openid: Option<String>,
    #[serde(rename = "sessionKey")]
    session_key: Option<String>,
    unionid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenService {}

impl TokenService {
    pub fn new() -> Self {
        TokenService {}
    }

    pub async fn jscode_to_session(
        &self,
        conf: &IntegrationConfig,
        req: CodeToSessionRequest,
    ) -> Result<CodeToSessionResponse, Box<dyn std::error::Error>> {
        let code = req.code.ok_or("code required")?;
        let url = format!("https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code", conf.wechat_appid, conf.wechat_secret, code);
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
