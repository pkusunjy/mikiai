use crate::config;

use wechat_pay_rust_sdk::model::{AmountInfo, JsapiParams, PayerInfo};
use wechat_pay_rust_sdk::pay::WechatPay;
use wechat_pay_rust_sdk::response::JsapiResponse;

pub async fn pay() -> JsapiParams {
    let wechat_pay = WechatPay::from_env();
    let body = wechat_pay.jsapi_pay(JsapiParams::new(
        "MikiAi会员购买",
        "1243243",
        1.into(),
        "open_id".into(),
    ));
    JsapiParams::new(
        "description",
        "out_trade_no",
        AmountInfo { total: 1 },
        PayerInfo {
            openid: String::from("123"),
        },
    )
}

#[derive(Debug)]
struct WechatPayService {}
