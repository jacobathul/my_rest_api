use sqlx::PgPool;
use crate::models::{User, NewUser, UpdatUser, Book, NewBook, UpdateBook};

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let result = sqlx::query_as!(User, 
        r#"
        SELECT id, name, email
        FROM users
        ORDER BY id
        "#
    )
    .fetch_all(pool).await;
    result
}

pub async fn add_user(pool: &PgPool, new_user: &NewUser) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User, 
        r#"
        INSERT INTO users (name, email)
        VALUES ($1, $2)
        RETURNING *
        "#,
        new_user.name, new_user.email
    )
    .fetch_one(pool)
    .await
}

// add a new replace user function take id as parameter
pub async fn replace_user(pool: &PgPool, user_id: i32, new_user: &NewUser) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User, 
        r#"
        UPDATE users
        SET name = $1, email = $2
        WHERE id = $3
        RETURNING *
        "#,
        new_user.name, new_user.email, user_id
    )
   .fetch_one(pool)
   .await
}
pub async fn update_user(pool: &PgPool, new_user_id: i32 ,new_user: &UpdatUser) -> Result<User, sqlx::Error> {
    let updated = sqlx::query_as!(User, 
        r#"
        UPDATE users
        SET name = COALESCE($2, name), email = COALESCE($3, email)
        WHERE id = $1
        RETURNING id, name, email
        "#,
        new_user_id, new_user.name, new_user.email
    ) // COALESCE is used to check if the value is null, if it is null then it will use the old value
    .fetch_one(pool).await; // Executes the query and returns the result
    updated
}

pub async fn delete_user(pool: &PgPool, id: i32) -> Result<User, sqlx::Error> {
    let deleted = sqlx::query_as!(User, 
        r#"
        DELETE FROM users
        WHERE id = $1
        RETURNING id, name, email
        "#,
        id
    )
    .fetch_one(pool).await;
    deleted
}

pub async fn get_books(pool: &PgPool) -> Result<Vec<Book>, sqlx::Error> {
    let result = sqlx::query_as::<_, Book>(
        r#"
        SELECT id, title, author
        FROM books
        ORDER BY id
        "#
    )
    .fetch_all(pool).await;
    result
}

pub async fn add_book(pool: &PgPool, new_book: &NewBook) -> Result<Book, sqlx::Error> {
    let added = sqlx::query_as!(Book,
        r#"
        INSERT INTO books (title, author)
        VALUES ($1, $2)
        RETURNING *
        "#,
        new_book.title, new_book.author
    )
    .fetch_one(pool).await;
    added
}

pub async fn replace_book(pool: &PgPool, book_id: i32, new_book: &NewBook) -> Result<Book, sqlx::Error> {
    sqlx::query_as!(Book,
        r#"
        UPDATE books
        SET title = $1, author = $2
        WHERE id = $3
        RETURNING *
        "#,
        new_book.title, new_book.author, book_id
    )
  .fetch_one(pool)
  .await
}
pub async fn update_book(pool: &PgPool, book_id: i32, book: &UpdateBook) -> Result<Book, sqlx::Error> {
    let updated = sqlx::query_as!(Book, 
        r#"
        UPDATE books
        SET title = COALESCE($2, title), author = COALESCE($3, author)
        WHERE id = $1
        RETURNING id, title, author
        "#,
        book_id, book.title, book.author
    )
    .fetch_one(pool).await;
    updated
}

pub async fn delete_book(pool: &PgPool, id: i32) -> Result<Book, sqlx::Error> {
    let deleted = sqlx::query_as!(Book, 
        r#"
        DELETE FROM books
        WHERE id = $1
        RETURNING id, title, author
        "#,
        id
    )
    .fetch_one(pool).await;
    deleted
}

//Generate a delete all items in the users table
pub async fn delete_all_users(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM users
        "#
    )
    .execute(pool).await?;
    Ok(())
}

//generate a delete all items in the books table
pub async fn delete_all_books(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM books
        "#
    )
    .execute(pool).await?;
    Ok(())
}
