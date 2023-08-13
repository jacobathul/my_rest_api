use actix_web::{web, App, HttpServer, Responder,web::Json};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use crate::models::{NewUser, UpdatUser, NewBook, UpdateBook};

pub mod models;
pub mod db;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url).await.expect("Failed to create pool.");


    HttpServer::new(move || {
        App::new()  
            .app_data(web::Data::new(db_pool.clone()))
            .route("/", web::get().to(index))
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(add_user_handler))
            .route("/users/{id}", web::put().to(replace_user_handler))
            .route("/users/{id}", web::patch().to(update_user_handler))
            .route("/users/{id}", web::delete().to(delete_user_handler))
            .route("/books", web::get().to(get_books))
            .route("/books", web::post().to(add_book_handler))
            .route("/books/{id}", web::put().to(replace_book_handler))
            .route("/books/{id}", web::patch().to(update_book_handler))
            .route("/books/{id}", web::delete().to(delete_book_handler))
            
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

async fn index() -> impl Responder {
    "Hello world!"
}

async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    let result = db::get_users(&pool).await;
    match result {
        Ok(users) => format!("{:?}", users),
        _ => "Error getting users".to_string(),
    }
}

async fn add_user_handler(pool: web::Data<PgPool>, new_user: Json<NewUser>) -> impl Responder {
    let result = db::add_user(&pool, &new_user).await;
    match result {
        Ok(user) => format!("Added user: {:?}", user),
        _ => "Error adding user".to_string(),
    }
}

async fn replace_user_handler(pool: web::Data<PgPool>, user_id: web::Path<i32>, user: Json<NewUser>) -> impl Responder {
    let result = db::replace_user(&pool, *user_id, &user).await;
    match result {
        Ok(user) => format!("Updated user: {:?}", user),
        _ => "Error updating user".to_string(),
    }
}

async fn update_user_handler(pool: web::Data<PgPool>, user_id: web::Path<i32>, user: Json<UpdatUser>) -> impl Responder {
    let result = db::update_user(&pool, *user_id, &user).await;
    match result {
        Ok(user) => format!("Updated user: {:?}", user),
        _ => "Error updating user".to_string(),
    }
}

async fn delete_user_handler(pool: web::Data<PgPool>, user_id: web::Path<i32>) -> impl Responder {
    let result = db::delete_user(&pool,*user_id).await;
    match result {
        Ok(user) => format!("Deleted user: {:?}", user),
        _ => "Error deleting user".to_string(),
    }
}

//all four handlers for books as
async fn get_books(pool: web::Data<PgPool>) -> impl Responder {
    let result = db::get_books(&pool).await;
    match result {
        Ok(books) => format!("{:?}", books),
        _ => "Error getting books".to_string(),
    }
}

async fn add_book_handler(pool: web::Data<PgPool>, new_book: Json<NewBook>) -> impl Responder {
    let result = db::add_book(&pool, &new_book).await;
    match result {
        Ok(book) => format!("Added book: {:?}", book),
        _ => "Error adding book".to_string(),
    }
}

async fn replace_book_handler(pool: web::Data<PgPool>, book_id: web::Path<i32>, book: Json<NewBook>) -> impl Responder {
    let result = db::replace_book(&pool, *book_id, &book).await;
    match result {
        Ok(user) => format!("Updated book: {:?}", user),
        _ => "Error updating user".to_string(),
    }
}

async fn update_book_handler(pool: web::Data<PgPool>, book_id: web::Path<i32>, book: Json<UpdateBook>) -> impl Responder {
    let result = db::update_book(&pool, *book_id, &book).await;
    match result {
        Ok(book) => format!("Updated book: {:?}", book),
        _ => "Error updating book".to_string(),
    }
}

async fn delete_book_handler(pool: web::Data<PgPool>, book_id: web::Path<i32>) -> impl Responder {
    let result = db::delete_book(&pool,*book_id).await;
    match result {
        Ok(book) => format!("Deleted book: {:?}", book),
        _ => "Error deleting book".to_string(),
    }
}




// Similarly you can create other routes for create, update, delete (CRUD)
