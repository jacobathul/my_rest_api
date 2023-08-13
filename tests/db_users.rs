extern crate my_rest_api;
use my_rest_api::models::{NewUser, UpdatUser};
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

async fn setup_test_users(pool: &PgPool) {
    // Insert user 1

    delete_all_users(&pool).await.expect("Failed to delete all users");

    let user1 = NewUser {
        name: "Test User 1".to_string(),
        email: "test1@example.com".to_string(),
    };
    add_user(&pool, &user1).await.expect("Failed to insert test user 1");

    // Insert user 2
    let user2 = NewUser {
        name: "Test User 2".to_string(),
        email: "test2@example.com".to_string(),
    };
    add_user(&pool, &user2).await.expect("Failed to insert test user 2");
}

async fn teardown_test_users(pool: &PgPool) {
    delete_all_users(&pool).await.expect("Failed to delete all users");
}

#[tokio::test]
async fn test_get_users() {
    let pool = get_test_db_pool().await;

    // Populate the test database with test data
    setup_test_users(&pool).await;

    // Now call the function to test
    let result = get_users(&pool).await;

    // Assert that the returned users match the inserted users
    assert!(result.is_ok());
    let users = result.unwrap();
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].name, "Test User 1");
    assert_eq!(users[1].name, "Test User 2");

    teardown_test_users(&pool).await;
}

#[tokio::test]
async fn test_update_user() {
    let pool = get_test_db_pool().await;

    // Populate the test database with test data
    setup_test_users(&pool).await;

    let result = get_users(&pool).await;
    assert!(result.is_ok());
    let users = result.unwrap();
    assert_eq!(users.len(), 2);

    let user_name_changed_only = UpdatUser {
        name: Some("Test User 1 Updated".to_string()),
        email: None,
    };

     let user_email_changed_only = UpdatUser {
        name: None,
        email: Some("testUser2Update.com".to_string()),
    };

   let result = update_user(&pool, users[0].id, &user_name_changed_only).await;
    assert!(result.is_ok());
    // Assert that the returned users match the updated users
    let user = result.unwrap();
    assert_eq!(user.name, "Test User 1 Updated");
    assert_eq!(user.email, users[0].email);

    let result = update_user(&pool, users[1].id, &user_email_changed_only).await;
    assert!(result.is_ok());
    // Assert that the returned users match the updated users
    let user = result.unwrap();
    assert_eq!(user.name, users[1].name);
    assert_eq!(user.email, "testUser2Update.com".to_string());

    
    teardown_test_users(&pool).await;

}

//generate a test for the delete_user function
#[tokio::test]
async fn test_delete_user() {
    let pool = get_test_db_pool().await;

    setup_test_users(&pool).await;

    let result = get_users(&pool).await;
    assert!(result.is_ok());
    let users = result.unwrap();
    assert_eq!(users.len(), 2);

    let result = delete_user(&pool, users[0].id).await;
    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.id, users[0].id);
    assert_eq!(user.name, users[0].name);
    assert_eq!(user.email, users[0].email);

    let result = get_users(&pool).await;
    assert!(result.is_ok());
    let users = result.unwrap();
    assert_eq!(users.len(), 1);

    teardown_test_users(&pool).await;
}

//generate to test for replacing users
#[tokio::test]
async fn test_replace_user() {
    let pool = get_test_db_pool().await;

    setup_test_users(&pool).await;

    let result = get_users(&pool).await;
    assert!(result.is_ok());
    let users = result.unwrap();
    assert_eq!(users.len(), 2);

    let new_user = NewUser {
        name: "New User".to_string(),
        email: "NewUser@example.com".to_string()
    };

    let result = replace_user(&pool, users[1].id, &new_user).await;
    assert!(result.is_ok());
    let replaced_user = result.unwrap();
    assert_eq!(replaced_user.id, users[1].id);
    assert_eq!(replaced_user.name, new_user.name);
    assert_eq!(replaced_user.email, new_user.email);

    let result = get_users(&pool).await;
    assert!(result.is_ok());
    let users = result.unwrap();
    assert_eq!(users.len(), 2);
    assert_eq!(users[1].name, new_user.name);
    assert_eq!(users[1].email, new_user.email);
    teardown_test_users(&pool).await;
}