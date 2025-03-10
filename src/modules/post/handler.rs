use crate::AppState;

use super::post_models::{NewPost,Post,UpdatePost};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use r2d2_redis::redis::Commands;
use serde_json::json;
use sqlx::{query, query_as};

#[get("/getall")]
pub async fn get_all_post(data:web::<AppState>) -> impl Responder {
    match posts {
        Ok(posts) => {
            HttpResponse::Ok().json(json!({
                "status": "ok",
                "data": posts,
                "source": "database"
            }))
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Something bad happened when fetching all posts"
            }))
        }
    }
}
