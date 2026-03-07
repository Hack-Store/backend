use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use std::env;
use tokio_postgres::types::ToSql;
use tokio_postgres::NoTls;
use uuid::{timestamp::ClockSequence, ContextV7, Timestamp, Uuid};
mod logger;
mod tools;

use crate::logger::logger::{LogLevel, Logger};
use crate::tools::tools::{spawn_connection_thread};


lazy_static!(
    static ref DB_URL: String = env::var("DB_URL").unwrap();
    static ref PORT: String = env::var("DB_PORT").unwrap();
    static ref USER: String = env::var("DB_USER").unwrap();
    static ref PASSWORD: String = env::var("DB_PASSWORD").unwrap();
);

const LOGGER: Logger = Logger {  };


#[post("/api/v1/add_user")]
async fn add_user(req_body: String) -> impl Responder {
    LOGGER.log(&format!("Received request: {}", req_body), LogLevel::INFO);
    let json_body: serde_json::Result<serde_json::Value> = serde_json::from_str(&req_body);
    let json_body = match json_body {
        Ok(value) => value,
        Err(e) => {
            LOGGER.log(&format!("Error: {}", e), LogLevel::ERROR);
            return HttpResponse::BadRequest().body("Invalid JSON");
        }
    };

    let config = format!("host={} user={} password={} port={}", *DB_URL, *USER, *PASSWORD, *PORT);
    let conn = tokio_postgres::connect(&*config, NoTls).await;
    let (client, connection) = match conn {
        Ok(value) => value,
        Err(e) => {
            LOGGER.log(&format!("Error: {}", e), LogLevel::CRITICAL);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };
    let _handler = spawn_connection_thread(connection);

    // check if the user already exists
    let stmt = client.prepare("SELECT * FROM \"User\" WHERE username = $1 OR email = $2").await;
    let stmt = match stmt {
        Ok(value) => value,
        Err(e) => {
            LOGGER.log(&format!("Error: {}", e), LogLevel::ERROR);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };
    let rows = client.query(&stmt, &[&json_body["username"].to_string(), &json_body["email"].to_string()]).await;
    let rows = match rows {
        Ok(value) => value,
        Err(e) => {
            LOGGER.log(&format!("Error: {}", e), LogLevel::ERROR);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };
    if rows.len() > 0 {
        LOGGER.log("User already exists", LogLevel::ERROR);
        return HttpResponse::Conflict().body("User already exists");
    }

    let context = ContextV7::new();
    let ts = Timestamp::now(&context);
    let id = Uuid::new_v7(ts);

    let stmt = client.prepare("INSERT INTO \"User\" (id, username, email, name, \"avatarUrl\") VALUES ($1, $2, $3, $4, $5)").await;
    let stmt = match stmt {
        Ok(value) => value,
        Err(e) => {
            LOGGER.log(&format!("Error: {}", e), LogLevel::ERROR);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };

    let exec = client.execute(&stmt, &[&id.to_string(), &json_body["username"].to_string(), &json_body["email"].to_string(), &json_body["name"].to_string(), &json_body["avatarUrl"].to_string()]).await;
    let _ = match exec {
        Ok(value) => value,
        Err(e) => {
            LOGGER.log(&format!("Error: {}", e), LogLevel::ERROR);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };

    LOGGER.log(&format!("User added successfully: {id}"), LogLevel::INFO);
    let response = format!("User added successfully: {}", id);
    HttpResponse::Ok().body(response)
}

#[post("/api/v1/add_app")]
async fn add_app(req_body: String) -> impl Responder {
    LOGGER.log(&format!("Received request: {}", req_body), LogLevel::INFO);
    let json_body: serde_json::Result<serde_json::Value> = serde_json::from_str(&req_body);
    let json_body = match json_body {
        Ok(value) => value,
        Err(e) => {
            LOGGER.log(&format!("Error: {}", e), LogLevel::ERROR);
            return HttpResponse::BadRequest().body("Invalid JSON");
        }
    };

    let config = format!("host={} user={} password={} port={}", *DB_URL, *USER, *PASSWORD, *PORT);
    let conn = tokio_postgres::connect(&*config, NoTls).await;
    let (client, connection) = match conn {
        Ok(value) => value,
        Err(e) => {
            LOGGER.log(&format!("Error: {}", e), LogLevel::CRITICAL);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };
    let _handler = spawn_connection_thread(connection);

    let stmt = client.prepare("INSERT INTO \"App\" (name, description, \"changeLog\", categories, \"sourceCode\", website, \"iconUrl\", \"authorId\", platform) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)").await.unwrap();

    let exec = client.execute(&stmt, &[&json_body["name"].to_string(), &json_body["description"].to_string(), &json_body["changeLog"].to_string(), &json_body["categories"].to_string(), &json_body["sourceCode"].to_string(), &json_body["website"].to_string(), &json_body["iconUrl"].to_string(), &json_body["authorId"].to_string(), &json_body["platform"].to_string()]).await;
    let _ = match exec {
        Ok(value) => value,
        Err(e) => {
            LOGGER.log(&format!("Error: {}", e), LogLevel::ERROR);
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };

    LOGGER.log("App added successfully", LogLevel::INFO);
    let response = "App added successfully";
    HttpResponse::Ok().body(response)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(add_user)
            .service(add_app)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

