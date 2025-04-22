use actix_web::{get, post, put, delete, web::{self, Data}, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: usize,
    name: String,
}

struct AppState {
    users: Mutex<Vec<User>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = Data::new(AppState {
        users: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(web::scope("/api")
                .service(get_users)
                .service(get_user)
                .service(create_user)
                .service(update_user)
                .service(delete_user)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/users")]
async fn get_users(data: Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    HttpResponse::Ok().json(&*users)
}

#[get("/users/{id}")]
async fn get_user(path: web::Path<usize>, data: Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let users = data.users.lock().unwrap();
    
    if let Some(user) = users.iter().find(|u| u.id == user_id) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

#[post("/users")]
async fn create_user(user: web::Json<User>, data: Data<AppState>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    users.push(user.into_inner());
    HttpResponse::Created().json(&users[users.len() - 1])
}

#[put("/users/{id}")]
async fn update_user(path: web::Path<usize>, user: web::Json<User>, data: Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let mut users = data.users.lock().unwrap();
    
    if let Some(index) = users.iter().position(|u| u.id == user_id) {
        users[index] = user.into_inner();
        HttpResponse::Ok().json(&users[index])
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

#[delete("/users/{id}")]
async fn delete_user(path: web::Path<usize>, data: Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let mut users = data.users.lock().unwrap();
    
    if let Some(index) = users.iter().position(|u| u.id == user_id) {
        users.remove(index);
        HttpResponse::Ok().body("User deleted successfully")
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}
