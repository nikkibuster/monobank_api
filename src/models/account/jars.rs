#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Jar {
    id: String,
    send_id: String,
    title: String,
    description: Option<String>,
    currency_code: i32,
    balance: f64,
    goal: Option<f64>,
}
