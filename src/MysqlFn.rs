use std::env;

use mysql::*;
use mysql::prelude::*;
use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
pub struct musiclist{
    user:String,
    name:String,
    date:String,
    public:bool
}
pub fn query_all(url:&str,table:&str)->Vec<(String,String,String,String,String,String,String,String,String,i32)>{
    //let url = "mysql://root:123456@localhost:3306/p_layer";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let query_all = format!(r"SELECT * FROM {}",table) ;

    let result: Vec<(String,String,String,String,String,String,String,String,String,i32)> = conn
        .query_map(query_all, |(user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number)| {
            (user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number)
        })
        .unwrap();
    
    result

}
pub fn query_user(url:&str,table:&str,token:&str)->Vec<(String,String,String,String,String,String,String,String,String,i32)>{
    //let url = "mysql://root:123456@localhost:3306/p_layer";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let query_all = format!(r"SELECT * FROM {} WHERE user_token='{}'",table,token) ;

    let result: Vec<(String,String,String,String,String,String,String,String,String,i32)> = conn
        .query_map(query_all, |(user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number)| {
            (user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number)
        })
        .unwrap();
    result

}
pub fn querymusiclist(user:String,public:&str,all:bool)->Vec<musiclist>{
    dotenv::from_filename(".env").expect("Failed to load .env file");
    let DATABASE_URL = env::var("DATABASE_URL").expect("SERVER_URL not found in .env file");
    let pool = Pool::new(DATABASE_URL).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let mut query_all = format!(r"SELECT * FROM music_list");
    if !all{
        query_all = format!(r"{} WHERE user='{}'",query_all,user);
    }
    if all&&public=="true"{
        query_all=format!("{} WHERE public=true",query_all);
    }
    else
    if all&&public=="false"{
        query_all=format!("{} WHERE public=false",query_all);
    }

    if !all&&public=="true"{
        query_all=format!("{} AND public=true",query_all);
    }
    else
    if !all&&public=="false"{
        query_all=format!("{} AND public=false",query_all);
    };
    let musiclist: Vec<musiclist> = conn
        .query_map(query_all, |(user, name, date, public)| {
            musiclist {
                user,
                name,
                date,
                public,
            }
        })
        .unwrap();
    musiclist

}

pub fn login(url:&str,table:&str,account:&str,password:&str)->Vec<(String,String,String,String,String,String,String,String,String,i32)>{
    //let url = "mysql://root:123456@localhost:3306/p_layer";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let query_all = format!(r"SELECT * FROM {} WHERE user_account='{}' AND user_password='{}'",table,account,password) ;

    let result: Vec<(String,String,String,String,String,String,String,String,String,i32)> = conn
        .query_map(query_all, |(user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number)| {
            (user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number)
        })
        .unwrap();
    result

}
