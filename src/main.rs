#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_multipart_form_data;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::ContentType;

use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, Repetition};

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
use std::io;
use rocket::http::RawStr;
use std::io::Write;

use rocket::response::status::Custom;

use rocket::tokio::fs::File as TokioFile;

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
    let mut items: Vec<(String,String,String,String,String,String,String,String,String,i32)> = Vec::new();
    let result: Vec<(String,String,String,String,String,String,String,String,String,i32)> = MysqlFn::query_all("mysql://root:123456@localhost:3306/p_layer","user_data");

    for (user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number) in result {
        items.push((user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number));
    }

    serde_json::to_string(&items).unwrap()
}

#[get("/query_user/<token>")]
async fn query_user(token:&str) -> String {
    // let header = Header::new("X-Custom-Header", "custom value");
    // assert_eq!(header.to_string(), "X-Custom-Header: custom value");
    let mut items: Vec<(String,String,String,String,String,String,String,String,String,i32)> = Vec::new();
    let result: Vec<(String,String,String,String,String,String,String,String,String,i32)> = MysqlFn::query_user("mysql://root:123456@localhost:3306/p_layer","user_data",token);

    for (user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number) in result {
        items.push((user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number));
    }

    serde_json::to_string(&items).unwrap()
}

#[get("/get_token/<account>/<password>")]
async fn get_token(account:&str,password:&str) -> String {
    // let header = Header::new("X-Custom-Header", "custom value");
    // assert_eq!(header.to_string(), "X-Custom-Header: custom value");
    let mut items: Vec<(String,String,String,String,String,String,String,String,String,i32)> = Vec::new();
    let result: Vec<(String,String,String,String,String,String,String,String,String,i32)> = MysqlFn::login("mysql://root:123456@localhost:3306/p_layer","user_data",account,password);

    for (user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number) in result {
        items.push((user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number));
    }

    serde_json::to_string(&items).unwrap()
}

#[get("/test", data="<value>")]
async fn test(value:&str)-> String{println!("test:{}",value);format!("test:{}",value)}
#[post("/upload", data = "<data>")]
async fn upload(data: rocket::Data<'_>) -> std::result::Result<String, std::io::Error> {
    let mut file = File::create("upload/pingfanh").await?;

    // 将二进制数据写入文件
    data.open(100000.kilobytes()).stream_to(&mut file).await?;

    Ok("文件上传成功".to_string())
}
// #[post("/upload", data = "<data>")]
// async fn upload(content_type: &ContentType, data: rocket::data::Data<'_>) -> &'static str
// {
//     let mut options = MultipartFormDataOptions::with_multipart_form_data_fields(
//         vec! [
//             MultipartFormDataField::file("photo").content_type_by_string(Some(mime::IMAGE_STAR)).unwrap(),
//             MultipartFormDataField::raw("fingerprint").size_limit(4096),
//             MultipartFormDataField::text("name"),
//             MultipartFormDataField::text("email").repetition(Repetition::fixed(3)),
//             MultipartFormDataField::text("email"),
//         ]
//     );

//     let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();

//     let photo = multipart_form_data.files.get("photo"); // Use the get method to preserve file fields from moving out of the MultipartFormData instance in order to delete them automatically when the MultipartFormData instance is being dropped
//     let fingerprint = multipart_form_data.raw.remove("fingerprint"); // Use the remove method to move raw fields out of the MultipartFormData instance (recommended)
//     let name = multipart_form_data.texts.remove("name"); // Use the remove method to move text fields out of the MultipartFormData instance (recommended)
//     let email = multipart_form_data.texts.remove("email");

//     if let Some(file_fields) = photo {
//         let file_field = &file_fields[0]; // Because we only put one "photo" field to the allowed_fields, the max length of this file_fields is 1.

//         let _content_type = &file_field.content_type;
//         let _file_name = &file_field.file_name;
//         let _path = &file_field.path;
//         std::fs::copy(_path, "/update/photo.png").unwrap();

//         // You can now deal with the uploaded file.
//     }

//     if let Some(mut raw_fields) = fingerprint {
//         let raw_field = raw_fields.remove(0); // Because we only put one "fingerprint" field to the allowed_fields, the max length of this raw_fields is 1.

//         let _content_type = raw_field.content_type;
//         let _file_name = raw_field.file_name;
//         let _raw = raw_field.raw;

//         // You can now deal with the raw data.
//     }

//     if let Some(mut text_fields) = name {
//         let text_field = text_fields.remove(0); // Because we only put one "text" field to the allowed_fields, the max length of this text_fields is 1.

//         let _content_type = text_field.content_type;
//         let _file_name = text_field.file_name;
//         let _text = text_field.text;

//         // You can now deal with the text data.
//     }

//     if let Some(text_fields) = email {
//         for text_field in text_fields { // We put "email" field to the allowed_fields for two times and let the first time repeat for 3 times, so the max length of this text_fields is 4.
//             let _content_type = text_field.content_type;
//             let _file_name = text_field.file_name;
//             let _text = text_field.text;

//             // You can now deal with the text data.
//         }
//     }

//     "ok"
// }

#[post("/user/create",data="<user>")]
async fn create_user(user:&str){
    let user_data=json::str2json(user);
    println!("{}",user_data);
    //let mut items: Vec<(i32, String, String, String)> = Vec::new();
    let url = "mysql://root:123456@localhost:3306/p_layer";
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
    let mut user_data: Vec<(String)> = Vec::new();
    println!("user: {}", user);
    let json_value: serde_json::Value = serde_json::from_str(user).unwrap();

    let url = "mysql://root:123456@localhost:3306/p_layer";
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
        if let Some(user_info) = json_value[0].get("user_info") {
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
    .mount("/api", routes![query_all_user,upload,create_user,query_user,test,update_user,get_token])
    //.mount("/", FileServer::from(relative!("static")))
}