use crate::models::{classroom::Classroom, day::Day, lesson::Lesson, week::Week};
use actix_web::{get, HttpResponse, Responder};

use crate::models::teacher;

#[get("/current_week")]
pub async fn hello() -> impl Responder {
    let classrooms = vec![Classroom {
        number: String::from("228"),
    }];

    let teachers = vec![teacher::Teacher {
        name: String::from("Vasyl Filipovich"),
        entitlement: String::from("Prof"),
    }];

    let lessons = vec![Lesson {
        title: String::from("Lorem ipsum"),
        time_start: String::from("11:00"),
        time_end: String::from("12:30"),
        teachers: teachers,
        classrooms: classrooms,
    }];

    let week = Week {
        title: String::from("Hello!"),
        days: vec![Day {
            day: String::from("monday"),
            lessons: lessons,
        }],
    };
    HttpResponse::Ok().json(week)
}
