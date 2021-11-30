use crate::models::lesson::Lesson;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Day {
    pub day: String,
    pub lessons: Vec<Lesson>,
}