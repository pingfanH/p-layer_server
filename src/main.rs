#[macro_use] extern crate rocket;

use rocket::http::Header;
use rocket::Request;
use mysql::*;
use mysql::prelude::*;
// use rocket::data::{self, FromData};
// use rocket::Data;
// use std::io::Read;
// use rocket_contrib::json::Json;
// use std::{io};
// use rocket::tokio::time::{sleep, Duration};
// use rocket::tokio::task::spawn_blocking;
// use rocket::http::private::Array;
// use serde_json::to_string;
// use serde_json::{Result, Value};
// use rocket::response::status;
//use rocket::fs::{FileServer, relative};
mod json;
mod MysqlFn;
//#[derive(Debug, PartialEq, Eq)]


#[get("/query_all_user")]
async fn query_all_user() -> String {
    let mut items: Vec<(String,String,String,String,String,String,String,String,i32)> = Vec::new();
    let result: Vec<(String,String,String,String,String,String,String,String,i32)> = MysqlFn::query_all("mysql://root:123456@localhost:3306/p_layer","user_data");

    for (user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number) in result {
        items.push((user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number));
    }

    serde_json::to_string(&items).unwrap()
}

#[get("/query_user/<user>")]
async fn query_user(user:&str) -> String {
    // let header = Header::new("X-Custom-Header", "custom value");
    // assert_eq!(header.to_string(), "X-Custom-Header: custom value");
    let mut items: Vec<(String,String,String,String,String,String,String,String,i32)> = Vec::new();
    let result: Vec<(String,String,String,String,String,String,String,String,i32)> = MysqlFn::query_user("mysql://root:123456@localhost:3306/p_layer","user_data",user);

    for (user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number) in result {
        items.push((user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number));
    }

    serde_json::to_string(&items).unwrap()

}

#[get("/test", data="<value>")]
async fn test(value:&str)-> String{println!("test:{}",value);format!("test:{}",value)}

#[post("/upload")]
async fn upload(){
}
#[post("/user/create",data="<user>")]
async fn create_user(user:&str){
    let user_data=json::str2json(user);
    println!("{}",user_data);
    //let mut items: Vec<(i32, String, String, String)> = Vec::new();
    let url = "mysql://root:123456@localhost:3306/p_layer";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    //println!("item_iditem_id{}",items_data["item_id"]);

    let str1=user_data["user_id"].as_str().unwrap();
    let str2=user_data["user_account"].as_str().unwrap();
    let str3=user_data["user_password"].as_str().unwrap();
    let str4=user_data["user_name"].as_str().unwrap();
    let str5=user_data["user_age"].as_str().unwrap();
    let str6=user_data["user_info"].as_str().unwrap();
    let str7=user_data["user_sign_date"].as_str().unwrap();
    let str8=user_data["user_music_number"].to_string();
    let query_all: String =format!(r"
    INSERT INTO user_data (user_id, user_account, user_password,user_name,user_age,user_info,user_sign_date,user_music_number) 
    VALUES ('{}','{}','{}','{}','{}','{}','{}',{});
    ",str1,str2,str3,str4,str5,str6,str7,str8) ;
    println!("{}",query_all);
    conn.query_drop(query_all);
}
#[post("/user/update",data="<user>")]
async fn update_user(user:&str){
    let mut body_data: Vec<(String,String)> = Vec::new();
    println!("user: {}", user);
    let json_value: serde_json::Value = serde_json::from_str(user).unwrap();

    println!("Original JSON string: {}", json_value);
    //let mut items: Vec<(i32, String, String, String)> = Vec::new();
    let url = "mysql://root:123456@localhost:3306/p_layer";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    //println!("item_iditem_id{}",items_data["item_id"]);
    
    let query_all: String =format!(r"
    UPDATE user_data SET user_name='平凡H' WHERE user_account='2146265126';
    ");
}

#[get("/<_..>")]
async fn everything() ->String{format!("你访问这里干什么")}
// #[get("/<file..>")]
// fn files(file: PathBuf) -> Option<NamedFile> {
//     NamedFile::open(Path::new("static/").join(file)).ok()
// } 
#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![everything])
    .mount("/api", routes![query_all_user,upload,create_user,query_user,test,update_user])
    //.mount("/", FileServer::from(relative!("static")))
}