use r2d2_redis::r2d2::Pool;
use sqlx::query_as;

use crate::DbAppState;

use super::post_models::Post;

pub async fn get_all_post_query(db: &DbAppState)->Result<Post,sqlx::error> {
    match query_as!(
        Post,
        r#"SELECT * FROM post ORDER BY id"#,
    )
    .fetch_all(db)
    .await{
        Ok(post)=>Ok(post),
        Err(error)=>{
            Err(error)
        }
    }
}