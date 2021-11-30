use crate::teacher::Teacher;
use crate::models::classroom::Classroom;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Lesson {
    pub title: String,
    pub time_start: String,
    pub time_end: String,
    pub teachers: Vec<Teacher>,
    pub classrooms: Vec<Classroom>,
}
