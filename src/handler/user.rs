use serde::Deserialize;
use validator::Validate;
use actix_web::{Responder, web};

use crate::conf;
use crate::utility::db;
use crate::utility::result::*;
use crate::model::UserModel;
use crate::dao::user as userDao;
use crate::utility::context::Context;

#[derive(Debug, Validate, Deserialize)]
pub struct LoginInput {
    #[validate(length(max = 10, message="username must be less than 10 chars."))]
    pub username: Option<String>,
    #[validate(length(min = 6, message="password must be more than 6 chars."))]
    pub password: Option<String>,
}

pub async fn login(mut ctx: Context, input: web::Json<LoginInput>) -> impl Responder {
    // check user is login, give a user id
    let user_id: i32 = ctx.get("user_id").map(|i| *i).unwrap_or(0_i32);

    // if user id exists, then user is logined
    if user_id > 0 {
        println!("user is login, user id is: {}", user_id);
    }
    
    // validate json input
    if let Err(e) = input.validate() {
        return system("inputs invalid", Some(&e)).data(e).json();
    }

    // perform some test below with mysql

    let user = match userDao::find_by_username("wang").await {
        Ok(u) => match u {
            Some(u) => u,
            None => return user_not_found("not found user", None).json(),
        },
        Err(e) => return system("find user exception", Some(&*e)).json(),
    };

    // perform some test below with redis

    // save to redis
    let redis_key = format!("{}{}", conf::defs::redis::USER_INFO, user.id);

    let b_suc = db::redis::cache_set(&redis_key, serde_json::json!(user).to_string()).await;
    if b_suc == false {
        return system("redis cache set user info failed", None).json();
    }

    // get from redis
    let user_cache = db::redis::cache_get(&redis_key).await;

    if user_cache.is_empty() {
        return system("redis cache get user info failed", None).json();
    }

    let user: UserModel = serde_json::from_str(&user_cache).unwrap();

    // del from redis
    let del_suc = db::redis::cache_del(&redis_key).await;

    if del_suc == false {
        return system("redis cache del user info failed", None).json();
    }

    Success::data(user).json()
}
