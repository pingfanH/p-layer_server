#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

extern crate rocket_multipart_form_data;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::ContentType;

use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, Repetition};
use rocket::fs::NamedFile;
use std::path::PathBuf;
use mysql::*;
use mysql::prelude::*;
use rocket::Data;
use rocket::data::{ByteUnit, DataStream, ToByteUnit};
use rocket::futures::SinkExt;
use rocket::tokio::io::AsyncWriteExt;
use crate::rocket::tokio::io::AsyncReadExt;
use rocket::http::Status;
use rocket::response::status;
use rocket::tokio::fs::File;
use std::path::Path;
use std::{io, fs, env};
use std::env::var;
use std::fmt::format;
use rocket::http::RawStr;
use std::io::Write;
use chrono::format::StrftimeItems;
use chrono::Local;

use rocket::response::status::Custom;

use rocket::tokio::fs::File as TokioFile;
use serde_json::json;
use crate::MysqlFn::{musiclist, querymusiclist};
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
mod Ultis;
//#[derive(Debug, PartialEq, Eq)]
#[derive(FromForm)]
struct Upload<'f> {
    file: TempFile<'f>
}

#[get("/query_all_user")]
async fn query_all_user() -> String {
    dotenv::from_filename(".env").expect("Failed to load .env file");
    let DATABASE_URL = env::var("DATABASE_URL").expect("SERVER_URL not found in .env file");
    let mut items: Vec<(String,String,String,String,String,String,String,String,String,i32)> = Vec::new();
    let result: Vec<(String,String,String,String,String,String,String,String,String,i32)> = MysqlFn::query_all(&DATABASE_URL,"user_data");

    for (user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number) in result {
        items.push((user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number));
    }

    serde_json::to_string(&items).unwrap()
}

#[get("/query_user/<token>")]
async fn query_user(token:&str) -> String {
    dotenv::from_filename(".env").expect("Failed to load .env file");
    let DATABASE_URL = env::var("DATABASE_URL").expect("SERVER_URL not found in .env file");
    // let header = Header::new("X-Custom-Header", "custom value");
    // assert_eq!(header.to_string(), "X-Custom-Header: custom value");
    let mut items: Vec<(String,String,String,String,String,String,String,String,String,i32)> = Vec::new();
    let result: Vec<(String,String,String,String,String,String,String,String,String,i32)> = MysqlFn::query_user(&DATABASE_URL,"user_data",token);

    for (user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number) in result {
        items.push((user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number));
    }

    serde_json::to_string(&items).unwrap()
}

#[get("/get_token/<account>/<password>")]
async fn get_token(account:&str,password:&str) -> String {
    dotenv::from_filename(".env").expect("Failed to load .env file");
    let DATABASE_URL = env::var("DATABASE_URL").expect("SERVER_URL not found in .env file");
    // let header = Header::new("X-Custom-Header", "custom value");
    // assert_eq!(header.to_string(), "X-Custom-Header: custom value");
    let mut items: Vec<(String,String,String,String,String,String,String,String,String,i32)> = Vec::new();
    let result: Vec<(String,String,String,String,String,String,String,String,String,i32)> = MysqlFn::login(&DATABASE_URL,"user_data",account,password);

    for (user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number) in result {
        items.push((user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number));
    }

    serde_json::to_string(&items).unwrap()
}

#[get("/test", data="<value>")]
async fn test(value:&str)-> String{println!("test:{}",value);format!("test:{}",value)}
#[post("/upload/<user>/<filename>", data = "<data>")]
async fn upload(user:&str,filename:&str,data: rocket::Data<'_>) -> std::result::Result<String, std::io::Error> {
    let folder_name = format!("upload/{}",user);
    // 创建文件夹
    match fs::create_dir(folder_name) {
        Ok(_) => println!(""),
        Err(_) => println!(""),
    }

    let mut file = File::create(format!("upload/{}/{}",user,filename)).await?;
    
    // 将二进制数据写入文件
    data.open(100000.kilobytes()).stream_to(&mut file).await?;

    Ok("文件上传成功".to_string())
}
#[post("/uploadmusic/<p>/<user>/<filename>", data = "<data>")]
async fn uploadmusic(p:bool,user:&str,filename:&str,data: rocket::Data<'_>) -> std::result::Result<String, std::io::Error> {
    dotenv::from_filename(".env").expect("Failed to load .env file");
    let DATABASE_URL = env::var("DATABASE_URL").expect("SERVER_URL not found in .env file");
    let url = DATABASE_URL;
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();



    let local_time = Local::now();
    let ymd = "%Y%m%d";
    let hms = "%H%M%S";
    let date=format!("{}{}",ymd,hms);
    let date=StrftimeItems::new(date.as_str());
    let date=local_time.format_with_items(date);
    let date=date.to_string();

    //let musicid=MadeToken(10);
    let mut path:String="".to_string();
    let folder_name = format!("upload/{}/music",user);
    if p{
        path=format!("global/music/{}{}{}",user,filename,date);
    }
    else{
        path=format!("{}/music/{}{}",user,filename,date);
        // 创建文件夹
        match fs::create_dir(folder_name) {
            Ok(_) => println!(""),
            Err(_) => println!(""),
        }
    }
    let musicdata=format!("INSERT INTO music_list (user,name,date,public) VALUES('{}','{}','{}',{})",user,filename,date,p);


    let file_name=path;
    let mut file = File::create(format!("upload/{}",file_name)).await?;

    // 将二进制数据写入文件
    data.open(100000.kilobytes()).stream_to(&mut file).await?;
    conn.query_drop(musicdata);

    Ok("文件上传成功".to_string())
}
#[get("/download/<user>/<filename>")]
async fn download(user:&str,filename:&str) -> Option<NamedFile> {
    let path = PathBuf::from(format!("upload/{}/{}",user,filename));
    println!("{}",format!("upload/{}/{}",user,filename));
    NamedFile::open(path).await.ok()
}

#[get("/getmusiclist/<user>/<public>")]
async fn getmusiclist(user: String, public: &str)->String{
    let userclone=user.clone();
    let music_list: Vec<musiclist> = querymusiclist(user, public);

    let mut music_list_json: Vec<String>= vec![];

    for list in music_list {
       //println!("{:?}",list);
        let json_string = serde_json::to_value(list).unwrap();
        //println!("{}",json_string);
        let json_string=json_string.to_string();
        music_list_json.push(json_string);
    }
    println!("获取用户{}的音乐数据",&userclone);
    let music_list_json=music_list_json.join(",");
    let music_list_json=format!("[{}]",music_list_json);
    music_list_json
}
#[get("/getmusic/<user>/<name>/<public>")]
async fn getmusic(user: String,name:String, public: &str)->String{
    let userclone=user.clone();
    let music_list: Vec<musiclist> = querymusiclist(user, public);

    let mut music_list_json: Vec<String>= vec![];

    for list in music_list {
       //println!("{:?}",list);
        let json_string = serde_json::to_value(list).unwrap();
        //println!("{}",json_string);
        let json_string=json_string.to_string();
        music_list_json.push(json_string);
    }
    println!("获取用户{}的音乐数据",&userclone);
    let music_list_json=music_list_json.join(",");
    let music_list_json=format!("[{}]",music_list_json);
    let musicList: Vec<serde_json::Value> = serde_json::from_str(music_list_json.as_str()).unwrap();
    println!("{:?}",musicList);
    let music_list_json=Ultis::selectfromjson(musicList,"name",name).await;
    println!("{:?}",music_list_json);
    let text=format!("{},{},{},{}",music_list_json["user"],music_list_json["name"],music_list_json["date"],music_list_json["public"]);
    text.replace("\"","")
}

#[post("/user/create",data="<user>")]
async fn create_user(user:&str){
    let user_data=json::str2json(user);
    println!("{}",user_data);
    //let mut items: Vec<(i32, String, String, String)> = Vec::new();
    dotenv::from_filename(".env").expect("Failed to load .env file");
    let DATABASE_URL = env::var("DATABASE_URL").expect("SERVER_URL not found in .env file");
    let url = &DATABASE_URL;
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    //println!("item_iditem_id{}",items_data["item_id"]);
    let token=Ultis::MadeToken(30);
    let uid=Ultis::MadeUID(6);
    let account=user_data["user_account"].as_str().unwrap();
    let password=user_data["user_password"].as_str().unwrap();
    let name=user_data["user_name"].as_str().unwrap();
    let gender=user_data["user_gender"].as_str().unwrap();
    let age=user_data["user_age"].as_str().unwrap();
    let info=user_data["user_info"].as_str().unwrap();
    let sign_date=Ultis::GetDate();

    

    let query_all: String =format!(r"
    INSERT INTO user_data (user_token,user_id,user_account,user_password,user_name,user_gender,user_age,user_info,user_sign_date,user_music_number) 
    VALUES ('{}','{}','{}','{}','{}','{}','{}','{}','{}',0);
    ",token.as_str(),uid,account,password,name,gender,age,info,sign_date.as_str()) ;
    println!("{}",query_all);
    conn.query_drop(query_all);
}
#[post("/user/update",data="<user>")]
async fn update_user(user:&str){
    dotenv::from_filename(".env").expect("Failed to load .env file");
    let DATABASE_URL = env::var("DATABASE_URL").expect("SERVER_URL not found in .env file");

    let mut user_data: Vec<(String)> = Vec::new();
    println!("user: {}", user);
    let json_value: serde_json::Value = serde_json::from_str(user).unwrap();
    

    let url = &DATABASE_URL;
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    if let Some(user_token) = json_value[0].get("user_token") {
        let user_token = user_token.as_str().unwrap().replace("\"", "");
        if let Some(user_name) = json_value[0].get("user_name") {
           let user_name = user_name.as_str().unwrap().replace("\"", "");
            user_data.push(format!(r"UPDATE user_data SET user_name='{}' WHERE user_token='{}';", user_name,user_token));

        }
        if let Some(user_gender) = json_value[0].get("user_gender") {
           let user_gender = user_gender.as_str().unwrap().replace("\"", "");
            user_data.push(format!(r"UPDATE user_data SET user_gender='{}' WHERE user_token='{}';", user_gender,user_token));
        }
        if let Some(user_age) = json_value[0].get("user_age") {
            let user_age = user_age.as_str().unwrap().replace("\"", "");
            user_data.push(format!(r"UPDATE user_data SET user_age='{}' WHERE user_token='{}';", user_age,user_token));
        }
        if let Some(user_info) = json_value[0].get("user_intro") {
            let user_info = user_info.as_str().unwrap().replace("\"", "");
            user_data.push(format!(r"UPDATE user_data SET user_info='{}' WHERE user_token='{}';", user_info,user_token));
        }

        for data in user_data{
            println!("{:#?}", data);
            conn.query_drop(data);
        }
    }



    // println!("{}",json_value["user_name"]);

    //let mut items: Vec<(i32, String, String, String)> = Vec::new();
    dotenv::from_filename(".env").expect("Failed to load .env file");
    let DATABASE_URL = env::var("DATABASE_URL").expect("SERVER_URL not found in .env file");
    let url = &DATABASE_URL;
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
    .mount("/api", routes![query_all_user,upload,create_user,query_user,test,update_user,get_token,download,uploadmusic,getmusiclist,getmusic])
    //.mount("/", FileServer::from(relative!("static")))
}