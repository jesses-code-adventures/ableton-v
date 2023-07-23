#![allow(dead_code)]
use sqlx::{migrate::MigrateDatabase, query::Query, Pool, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

pub struct Database {
    pub db_url: String,
    pub connection: Option<Pool<Sqlite>>,
}

impl Database {
    pub async fn new() -> Database {
        let mut db = Database {
            db_url: String::from(DB_URL),
            connection: None,
        };
        db.create_database().await;
        db.connection = db.connect().await;
        db
    }

    pub async fn connect(&mut self) -> Option<Pool<Sqlite>> {
        let connection = SqlitePool::connect(self.db_url.as_str()).await.unwrap();
        println!("Connected to db");
        Some(connection)
    }

    pub async fn create_database(&self) {
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            println!("Creating database {}", DB_URL);
            match Sqlite::create_database(DB_URL).await {
                Ok(_) => println!("Success"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            println!("Database exists at {}", DB_URL);
        }
    }

    pub async fn execute_insert(
        &mut self,
        query: String,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        if self.connection.is_none() {
            self.connect().await.expect("database to connect");
        }
        let result: Query<Sqlite, _> = sqlx::query(query.as_str());
        result.execute(self.connection.as_ref().unwrap()).await
    }

    pub async fn execute_fetchone(
        &mut self,
        query: String,
    ) -> Result<sqlx::sqlite::SqliteRow, sqlx::Error> {
        if self.connection.is_none() {
            self.connect().await.expect("database to connect");
        }
        let result: Query<Sqlite, _> = sqlx::query(query.as_str());
        result.fetch_one(self.connection.as_ref().unwrap()).await
    }

    pub async fn execute_fetchall(
        &mut self,
        query: String,
    ) -> Result<Vec<sqlx::sqlite::SqliteRow>, sqlx::Error> {
        if self.connection.is_none() {
            self.connect().await.expect("database to connect");
        }
        let result: Query<Sqlite, _> = sqlx::query(query.as_str());
        result.fetch_all(self.connection.as_ref().unwrap()).await
    }
}

pub trait DatabaseModel {
    fn create_table_query(&self) -> String;
    fn insert_into_query(&self) -> String;
}
