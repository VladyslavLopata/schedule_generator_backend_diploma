use crate::models::day::Day;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Week {
    pub title: String,
    pub days: Vec<Day>,
}