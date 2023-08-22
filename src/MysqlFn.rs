use mysql::*;
use mysql::prelude::*;

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
