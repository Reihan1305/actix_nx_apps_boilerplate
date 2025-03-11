use crate::{modules::post::services::PostServices, AppState};

use super::models::{NewPost,Post,UpdatePost};
use actix_web::{delete, get, patch, post, web::{self, scope, ServiceConfig, Path}, HttpResponse, Responder};
use serde_json::json;
use crate::utils::response_builder::PostResponse;

    #[get("/getall")]
    pub async fn get_all_post(data: web::Data::<AppState>) -> impl Responder {
        let post_services= PostServices::get_all_post_query(&data.db).await;    
        match post_services {
            Ok(posts)=> {
                let data = json!({"data":posts});
                PostResponse::ok_response(data)
            },
            Err(error)=>{
                if error.contains("not found"){
                    PostResponse::not_found_response()
                }else{
                    PostResponse::internal_error_response(error)
                }
            }
        }
}

    #[get("/getone/post_id}")]
    pub async fn get_one_post(data: web::Data::<AppState>,path: Path<i32>) -> impl Responder{
        let post_id = path.into_inner();

        let post_services= PostServices::get_post_by_id(&data.db, post_id).await;
        
        match post_services{
            Ok(data)=>{
                let value = json!({"data":data});
                return PostResponse::ok_response(value)
            },
            Err(error)=>{
                return PostResponse::internal_error_response(error)
            }
        }
    }

    pub fn post_config(config: &mut ServiceConfig){
        config.service(
            scope("/post")
            .service(get_all_post)
            .service(get_one_post)
        );
    }