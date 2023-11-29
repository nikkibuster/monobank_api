#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Jar {
    pub id: String,
    pub send_id: String,
    pub title: String,
    pub description: Option<String>,
    pub currency_code: i32,
    pub balance: f64,
    pub goal: Option<f64>,
}
