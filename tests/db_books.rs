extern crate my_rest_api;
use my_rest_api::models::{NewBook, UpdateBook};
use my_rest_api::db::*;

use sqlx::PgPool;
use std::env;

// This will ensure the .env is loaded when the tests module is loaded.

// This is just a utility function to get a new database pool connection to the test database.
async fn get_test_db_pool() -> PgPool {
    
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST must be set"); // you should set this env variable to your test DB
    PgPool::connect(&database_url).await.expect("Failed to connect to test database")
}

async fn setup_test_books(pool: &PgPool) {
    // Insert Book 1

    delete_all_books(&pool).await.expect("Failed to delete all Books");

    let book1 = NewBook {
        title: "War and Peace".to_string(),
        author: "Leo Tolstoy".to_string(),
    };
    add_book(&pool, &book1).await.expect("Failed to insert test Book 1");

    // Insert Book 2
    let book2 = NewBook {
        title: "The Alchemist".to_string(),
        author: "Paolo Coelho".to_string(),
    };
    add_book(&pool, &book2).await.expect("Failed to insert test Book 2");
}

async fn teardown_test_books(pool: &PgPool) {
    delete_all_books(&pool).await.expect("Failed to delete all Books");
}

#[tokio::test]
async fn test_get_books() {
    let pool = get_test_db_pool().await;

    // Populate the test database with test data
    setup_test_books(&pool).await;

    // Now call the function to test
    let result = get_books(&pool).await;
    if let Err(e) = &result {
        println!("Error: {:?}", e);
    }
    // Assert that the returned Books match the inserted Books
    assert!(result.is_ok());
    let books = result.unwrap();
    assert_eq!(books.len(), 2);
    assert_eq!(books[0].author, "Leo Tolstoy");
    assert_eq!(books[1].author, "Paolo Coelho");

    teardown_test_books(&pool).await;
}

#[tokio::test]
async fn test_update_book() {
    let pool = get_test_db_pool().await;

    // Populate the test database with test data
    setup_test_books(&pool).await;

    let result = get_books(&pool).await;
    assert!(result.is_ok());
    let books = result.unwrap();
    assert_eq!(books.len(), 2);

    let book_title_changed_only = UpdateBook {
        title: Some("War Updated".to_string()),
        author: None,
    };

     let book_author_changed_only = UpdateBook {
        title: None,
        author: Some("Paolo updated".to_string()),
    };

   let result = update_book(&pool, books[0].id, &book_title_changed_only).await;
    assert!(result.is_ok());
    // Assert that the returned Books match the updated Books
    let book = result.unwrap();
    assert_eq!(book.title, "War Updated".to_string());
    assert_eq!(book.author, books[0].author);

    let result = update_book(&pool, books[1].id, &book_author_changed_only).await;
    assert!(result.is_ok());
    // Assert that the returned Books match the updated Books
    let book = result.unwrap();
    assert_eq!(book.title, books[1].title);
    assert_eq!(book.author, "Paolo updated".to_string());

    
    teardown_test_books(&pool).await;

}

#[tokio::test]
async fn test_delete_book() {
    let pool = get_test_db_pool().await;

    setup_test_books(&pool).await;

    let result = get_books(&pool).await;
    assert!(result.is_ok());
    let books = result.unwrap();
    assert_eq!(books.len(), 2);

    let result = delete_book(&pool, books[0].id).await;
    assert!(result.is_ok());
    let book = result.unwrap();
    assert_eq!(book.id, books[0].id);
    assert_eq!(book.title, books[0].title);
    assert_eq!(book.author, books[0].author);

    let result = get_books(&pool).await;
    assert!(result.is_ok());
    let books = result.unwrap();
    assert_eq!(books.len(), 1);

    teardown_test_books(&pool).await;
}

#[tokio::test]
async fn test_replace_book() {
    let pool = get_test_db_pool().await;

    setup_test_books(&pool).await;

    let result = get_books(&pool).await;
    assert!(result.is_ok());
    let books = result.unwrap();
    assert_eq!(books.len(), 2);

    let new_book = NewBook {
        title: "Updated Title".to_string(),
        author: "Updated Author".to_string()
    };

    let result = replace_book(&pool, books[1].id, &new_book).await;
    assert!(result.is_ok());
    let replaced_book = result.unwrap();
    assert_eq!(replaced_book.id, books[1].id);
    assert_eq!(replaced_book.title, new_book.title);
    assert_eq!(replaced_book.author, new_book.author);

    let result = get_books(&pool).await;
    assert!(result.is_ok());
    let books = result.unwrap();
    assert_eq!(books.len(), 2);
    assert_eq!(books[1].title, new_book.title);
    assert_eq!(books[1].author, new_book.author);

    teardown_test_books(&pool).await;
}