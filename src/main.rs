use crate::models::{classroom::Classroom, day::Day, lesson::Lesson, week::Week};
use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

mod models;
use models::teacher;

#[get("/current_week")]
async fn hello() -> impl Responder {
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
        classrooms: classrooms
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().wrap(Cors::permissive()).service(hello))
        .bind("localhost:8080")?
        .run()
        .await
}
