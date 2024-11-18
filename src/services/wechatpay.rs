use crate::config::IntegrationConfig;
use crate::services::platform::{DataPlatformService, WhitelistUserData};
use crate::services::swagger::{YsCustomerSaveRequest, YsOrderSaveRequest};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, Read};
use time::OffsetDateTime;
use wechat_pay_rust_sdk::model::JsapiParams;
use wechat_pay_rust_sdk::pay::WechatPay;

#[derive(Debug, Deserialize)]
pub struct CheckAndPayRequest {
    openid: Option<String>,
    amount: Option<i32>,
    data_platform_order_type: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CheckAndPayResponse {
    timestamp: Option<String>,
    #[serde(rename = "nonceStr")]
    nonce_str: Option<String>,
    package: Option<String>,
    #[serde(rename = "signType")]
    sign_type: Option<String>,
    #[serde(rename = "paySign")]
    pay_sign: Option<String>,
}

impl CheckAndPayResponse {
    pub fn new() -> Self {
        CheckAndPayResponse {
            timestamp: None,
            nonce_str: None,
            package: None,
            sign_type: None,
            pay_sign: None,
        }
    }
}

#[derive(Debug)]
pub struct WechatPayService {
    customer_save_url: String,
    order_save_url: String,
    edit_order_url: String,
    wechat_pay: WechatPay,
}

impl WechatPayService {
    pub fn new(
        integration_conf: &IntegrationConfig,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let private_key = std::fs::read_to_string("/home/work/cert/apiclient_key.pem")?;
        Ok(WechatPayService {
            customer_save_url: integration_conf
                .gen_utility_url_by_route("/utility-project/ysCustomer/save"),
            order_save_url: integration_conf
                .gen_utility_url_by_route("/utility-project/ysOrder/save"),
            edit_order_url: integration_conf
                .gen_utility_url_by_route("/utility-project/ysOrder/editOrderStatus"),
            wechat_pay: WechatPay::new(
                integration_conf.wechat_appid.clone(),
                integration_conf.wechat_mch_id.clone(),
                private_key,
                integration_conf.wechat_serial_no.clone(),
                integration_conf.wechat_v3_key.clone(),
                String::from("https://mikiai.tuyaedu.com:8124/wx_payment_notify/jsapi_notify_url"),
            ),
        })
    }
    pub async fn jsapi(
        &self,
        data: CheckAndPayRequest,
        data_platform_srv: &DataPlatformService,
    ) -> Result<CheckAndPayResponse, Box<dyn std::error::Error>> {
        let openid = data.openid.ok_or("openid required")?;
        // 按前端要求，调用/utility-project/ysCustomer/save保存用户某些信息, 不用关心返回值
        let customer_save_req = YsCustomerSaveRequest {
            member_type: Some("0"),
            username: Some(&openid),
        };
        info!(
            "[jsapi] url {} request:{:?}",
            self.customer_save_url, customer_save_req
        );
        let mut customer_save_resp = awc::Client::default()
            .post(&self.customer_save_url)
            .send_json(&customer_save_req)
            .await?;
        let customer_save_resp_body = String::from_utf8(customer_save_resp.body().await?.to_vec())?;
        info!(
            "[jsapi] url {} response body: {:?}",
            self.customer_save_url, customer_save_resp_body
        );
        // 生成out_trade_no，记录订单号
        let out_trade_no = get_random_str()?;
        // 按前端要求，调用/utility-project/ysCustomer/save保存订单, 不用关心返回值
        let order_save_req = YsOrderSaveRequest {
            order_code: Some(&out_trade_no),
            order_type: data.data_platform_order_type,
            username: Some(&openid),
        };
        info!(
            "[jsapi] url {} request:{:?}",
            self.order_save_url, order_save_req
        );
        let mut order_save_resp = awc::Client::default()
            .post(&self.order_save_url)
            .send_json(&order_save_req)
            .await?;
        let order_save_resp_body = String::from_utf8(order_save_resp.body().await?.to_vec())?;
        info!(
            "[jsapi] url {} response body: {:?}",
            self.order_save_url, order_save_resp_body
        );
        // 微信支付前，校验用户是否在白名单里，如果在，直接跳过后续支付逻辑
        let data_platform_order_type = data.data_platform_order_type.unwrap_or(0);
        if data_platform_order_type == 3 {
            let query_res = data_platform_srv
                .query_whitelist_user(WhitelistUserData::new_with_openid(openid.clone()))
                .await?;
            if query_res.len() > 0 {
                let cur_user = &query_res[0];
                let now_unix = OffsetDateTime::now_utc().unix_timestamp() as u64;
                let mut is_free_user = true;
                if let Some(status) = cur_user.status {
                    info!("[jsapi] openid:{} status {}", openid, status);
                    is_free_user = is_free_user && (status == 1);
                }
                match (cur_user.added_time, cur_user.expiration_date) {
                    (Some(added_time), Some(expiration_date)) => {
                        let is_time_valid = (added_time < now_unix) && (now_unix < expiration_date);
                        info!(
                            "[jsapi] openid:{} added_time:{} expire_time:{} now:{}",
                            openid, added_time, expiration_date, now_unix
                        );
                        if is_time_valid {
                            info!("[jsapi] openid:{} time check ok", openid);
                        } else {
                            info!("[jsapi] openid:{} time check not ok", openid);
                        }
                        is_free_user = is_free_user && is_time_valid;
                    }
                    (_, _) => {
                        warn!(
                            "[jsapi] openid:{} added_time or expiration_date not found",
                            openid
                        );
                    }
                }
                if is_free_user {
                    info!("[jsapi] openid:{} free user", openid);
                    let edit_order_req = YsOrderSaveRequest {
                        order_code: Some(&out_trade_no),
                        order_type: None,
                        username: None,
                    };
                    info!(
                        "[jsapi] url {} request:{:?}",
                        self.edit_order_url, edit_order_req
                    );
                    let mut edit_order_resp = awc::Client::default()
                        .post(&self.edit_order_url)
                        .send_json(&edit_order_req)
                        .await?;
                    let edit_order_resp_body =
                        String::from_utf8(edit_order_resp.body().await?.to_vec())?;
                    info!(
                        "[jsapi] url {} response body: {:?}",
                        self.edit_order_url, edit_order_resp_body
                    );
                    return Ok(CheckAndPayResponse::new());
                }
            }
        }
        // 创建prepay_id
        let amount = data.amount.ok_or(format!(
            "[jsapi] openid:{} need to pay, but no amount",
            openid
        ))?;
        let wechat_pay_body = self
            .wechat_pay
            .jsapi_pay(JsapiParams::new(
                "MikiAi会员购买",
                &out_trade_no,
                amount.into(),
                openid.as_str().into(),
            ))
            .await?;
        info!("[jsapi] openid:{} pay success", openid);
        let sign_data = wechat_pay_body
            .sign_data
            .ok_or(format!("[jsapi] openid:{} get sign_data failed", openid))?;
        let resp = CheckAndPayResponse {
            timestamp: Some(sign_data.timestamp),
            nonce_str: Some(sign_data.nonce_str),
            package: Some(sign_data.package),
            sign_type: Some(sign_data.sign_type),
            pay_sign: Some(sign_data.pay_sign),
        };
        info!("[jsapi] openid:{} resp:{:?}", openid, resp);

        Ok(resp)
    }
}

fn get_random_str() -> Result<String, io::Error> {
    let mut file = File::open("/dev/random")?;
    // 读取16字节随机数据
    let mut buf = [0u8; 16];
    file.read_exact(&mut buf)?;
    // 拼装字符串
    let mut result = String::new();
    for chunk in buf.chunks(4) {
        let value = u32::from_be_bytes(chunk.try_into().unwrap());
        result.push_str(&format!("{:08X}", value));
    }
    Ok(result)
}
