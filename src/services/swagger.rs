use serde::Serialize;

#[derive(Default, Debug, Serialize)]
#[serde(default)]
pub struct YsCustomerSaveRequest<'a> {
    #[serde(rename = "memberType")]
    pub member_type: Option<&'a str>,
    pub username: Option<&'a str>,
}

#[derive(Default, Debug, Serialize)]
#[serde(default)]
pub struct YsOrderSaveRequest<'a> {
    #[serde(rename = "orderCode")]
    pub order_code: Option<&'a str>,
    #[serde(rename = "orderType")]
    pub order_type: Option<i32>,
    pub username: Option<&'a str>,
}
