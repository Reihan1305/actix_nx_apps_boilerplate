use crate::DbAppState;

use super::{models::Post, query::PostQuery};
pub struct PostServices;

impl PostServices{
    pub async fn get_all_post_query(db: &DbAppState)->Result<Vec<Post>,String> {
        let post_query=  PostQuery::get_all_posts(db).await;

        match post_query{
            Ok(data)=> {
                if data.len() == 0{
                    Err(
                        String::from("data not found")
                    )
                }else {
                    Ok(data)
                }
            },
            Err(error)=>{
                Err(format!("{}",error))
            }
        }
    }

    pub async fn get_post_by_id(db:&DbAppState,post_id: i32) -> Result<Post,String>{
        let post_query=  PostQuery::get_post_by_id(db,post_id).await;

        match post_query{
            Ok(data)=> Ok(data),
            Err(error)=>{
                Err(format!("{}",error))
            }
        }
    }


}