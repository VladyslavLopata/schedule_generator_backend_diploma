use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Teacher {
    pub name: String,
    pub entitlement: String,
}