// mod modules;
mod utils;
use actix_web::{web, App, HttpServer, Responder};
use utils::response_builder::SignatureAccessToken;
use serde_json::json;

async fn ok_response() -> impl Responder {
    SignatureAccessToken::signature_success_response_builder("horeeeee".to_string())
}

async fn unauthorized_response() -> impl Responder{
    SignatureAccessToken::signature_unauthorize_response_builder()
}
#[actix_web::main]  
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(ok_response))
        .route("/error", web::get().to(unauthorized_response))

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


// use std::{
//     collections::HashMap,
//     env::{
//         var_os,
//         set_var,
//         var
//     }
// };
// use actix_cors::Cors;
// use actix_web::{http::header,middleware::Logger,{
//     get, 
//     HttpResponse, 
//     Responder, 
//     web::{self,scope}, 
//     App, 
//     HttpServer
// }};
// use lapin::{
//     BasicProperties,
//     types::FieldTable,
//     options::{
//     BasicPublishOptions, 
//     QueueDeclareOptions
//     }
// };

// use r2d2_redis::redis::Commands;
// use serde_json::json;
// use sqlx::{Pool, Postgres};


// pub type DbAppState = Pool<Postgres>;
// /// Shared state for Actix App
// pub struct AppState {
//     db:DbAppState
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     dotenv::dotenv().ok();
//     // Setup logger and environment variables
//     env_logger::init();
//     if var_os("RUST_LOG").is_none() {
//         set_var("RUST_LOG", "actix_web=info");
//     }
//     //get port from env
//     let port: u16 = var("PORT")
//                     .expect("cant get port from env")
//                     .parse::<u16>()
//                     .expect("cant convert port to u16");

//     // get database_url from env
//     let database_url: String = var("DATABASE_URL")
//                                .expect("cant get db url from env");

//     // create initial pool database
//     let pool: sqlx::Pool<sqlx::Postgres> = match sqlx::postgres::PgPoolOptions::new()
//         .min_connections(5)
//         .max_connections(50)
//         .connect(&database_url)
//         .await {
//             Ok(pg_pool)=> {
//                 println!("✅ Connection to the database is successful!");
//                 pg_pool
//             },
//             Err(err) => {
//             println!("failed to connect database: {}",err);
//             std::process::exit(1)
//             }
//         };

//     // print the status server and the port
//     println!("🚀 Server started successfully at port {:?}",port);
    
//     // Start Actix server
//     HttpServer::new(move || {
//         //configure the cors
//         let cors = Cors::default()
//             .allowed_origin("http://localhost:3000")
//             .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
//             .allowed_headers(vec![
//                 header::CONTENT_TYPE,
//                 header::AUTHORIZATION,
//                 header::ACCEPT,
//             ])
//             .supports_credentials();

//         App::new()
//             .app_data(web::Data::new(AppState {
//                 db: pool.clone(),
//             }))
//             .wrap(cors)
//             .wrap(Logger::default())
//             .service(
//                 scope("/api")
//             )
//     })
//     .bind(("0.0.0.0",port))?
//     .run()
//     .await
// }
