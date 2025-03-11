use sqlx::query_as;
use crate::DbAppState;

use super::models::Post;

pub struct PostQuery;

impl PostQuery{
    pub async fn get_all_posts(
        db: &DbAppState
    )-> Result<Vec<Post>,sqlx::Error>{
        let query = query_as!(
            Post,
            r#"SELECT * FROM posts ORDER BY id"#
        ).fetch_all(db).await;
    
        match query{
            Ok(data)=> Ok(data),
            Err(error)=>Err(error)
        }
    }


    pub async fn get_post_by_id(
        db: &DbAppState,
        id: i32
    ) -> Result<Post,sqlx::Error>{
        let query = query_as!(
            Post,
            r#"SELECT * FROM posts where id = $1"#,
            id
        ).fetch_one(db).await;
    
        match query{
            Ok(data)=> Ok(data),
            Err(error)=>Err(error)
        }
    }
}