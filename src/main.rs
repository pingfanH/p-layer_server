#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_multipart_form_data;
use MysqlFn::insert;
use rocket::figment::providers::{Toml, Format};
use rocket::fs::TempFile;
use rocket::fs::NamedFile;
use std::collections::HashMap;
use std::net::IpAddr;
use std::path::PathBuf;
use std::str::FromStr;
use mysql::*;
use mysql::prelude::*;
use rocket::data::{ToByteUnit};
use rocket::futures::SinkExt;
use rocket::tokio::io::AsyncWriteExt;
use crate::rocket::tokio::io::AsyncReadExt;
use rocket::tokio::fs::File;
use std::{fs, env};
use std::fmt::format;
use std::io::Write;
use chrono::format::StrftimeItems;
use chrono::Local;
use crate::MysqlFn::{musiclist, querymusiclist,update};
use crate::Ultis::{get_date, MadeUID};

mod json;
mod MysqlFn;
mod Ultis;
mod cors;
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
#[post("/upload/<path..>", data = "<data>")]
async fn upload(path:PathBuf,data: rocket::Data<'_>) -> std::result::Result<String, std::io::Error> {
    let path=path.to_str().unwrap();
    let mut file = File::create(format!("upload/{}",path)).await?;
    // 将二进制数据写入文件
    data.open(20000.kilobytes()).stream_to(&mut file).await?;

    Ok("文件上传成功".to_string())
}
#[post("/uploadmusic/<p>/<user>/<filename>", data = "<data>")]
async fn uploadmusic(p:bool,user:&str,filename:&str,data: rocket::Data<'_>) -> std::result::Result<String, std::io::Error> {
    dotenv::from_filename(".env").expect("Failed to load .env file");
    let DATABASE_URL = env::var("DATABASE_URL").expect("SERVER_URL not found in .env file");
    let url = DATABASE_URL;
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let date=get_date();
    let id=format!("{}{}",MadeUID(5),date);

    let musicdata=format!("INSERT INTO music_list (user,name,date,public,id) VALUES('{}','{}','{}',{},'{}')",user,filename,date,p,id);

    let mut file = File::create(format!("upload/music/{}",id)).await?;

    // 将二进制数据写入文件
    data.open(20000.kilobytes()).stream_to(&mut file).await?;
    conn.query_drop(musicdata).expect("INSERT WRONG");

    Ok(id)
}
#[get("/download/<path..>")]
async fn download(path:PathBuf) -> Option<NamedFile> {
    let pathstr=path.to_str().unwrap();
    let path = PathBuf::from(format!("upload/{}",pathstr));
    println!("{}",format!("upload/{}",pathstr));
    NamedFile::open(path).await.ok()
}

#[get("/downloadmusic/<filename>")]
async fn downloadmusic(filename:&str) -> Option<NamedFile> {
    let path = PathBuf::from(format!("upload/music/{}",filename));
    println!("{}",format!("upload/music/{}",filename));
    NamedFile::open(path).await.ok()
}

#[get("/getmusiclist/<user>/<public>")]
async fn getmusiclist(user: String, public: &str)->String{
    let userclone=user.clone();
    let music_list: Vec<musiclist> = querymusiclist(user, public,false);

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
#[get("/get_music_page/<id>")]
async fn get_music_page(id:&str)->String{
    let json=MysqlFn::query_music("id",id);
    json.to_string()
}
#[get("/get_pub_musics")]
async fn get_pub_musics()->String{
    let json=MysqlFn::query_music_rand(10);
    json.to_string()
}
#[get("/get_pub_playlists")]
async fn get_pub_playlists()->String{
    let json=MysqlFn::query_playlist_rand(4);
    json.to_string()
}
#[get("/get_play_list/<user>/<public>")]
async fn get_play_list(user:&str,public:&str)->String{
    let play_list=MysqlFn::queryplaylist(user, public);
    play_list.to_string()
}


#[get("/getmusiclistall/<public>")]
async fn getmusiclistall(public: &str)->String{

    let music_list: Vec<musiclist> = querymusiclist("user".to_string(), public,true);

    let mut music_list_json: Vec<String>= vec![];

    for list in music_list {
        //println!("{:?}",list);
        let json_string = serde_json::to_value(list).unwrap();
        //println!("{}",json_string);
        let json_string=json_string.to_string();
        music_list_json.push(json_string);
    }
    println!("获取音乐数据");
    let music_list_json=music_list_json.join(",");
    let music_list_json=format!("[{}]",music_list_json);
    music_list_json
}

#[get("/getmusic/<user>/<name>/<public>")]
async fn getmusic(user: String,name:String, public: &str)->String{
    let userclone=user.clone();
    let music_list: Vec<musiclist> = querymusiclist(user, public,false);

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
    match fs::create_dir(format!("upload/{}",account)){
        Ok(ok)=>ok,
        Err(_)=>eprintln!("已有文件夹"),
    };
    

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
    let database_url = env::var("DATABASE_URL").expect("SERVER_URL not found in .env file");

    let mut user_data: Vec<String> = Vec::new();
    println!("user: {}", user);
    let json_value: serde_json::Value = serde_json::from_str(user).unwrap();

    let pool = Pool::new(&database_url).unwrap();
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

}

#[post("/new_play_list",data = "<data>")]
async fn new_play_list(data:String) -> String{
    let mut map:HashMap<String,serde_json::Value> = serde_json::from_str(data.as_str()).unwrap();

    map.insert("date".to_string(),serde_json::Value::String(get_date()));
    let id=format!("{}{}",MadeUID(5),get_date());
    map.insert("id".to_string(),serde_json::Value::String(id.clone()));
    map.insert("content".to_string(),serde_json::Value::String("".to_string()));
    println!("{map:?}");
    insert(map,"play_list");
    id.to_string()
}
#[post("/update_play_list",data = "<data>")]
async fn update_play_list(data:String) -> String{
    let mut map:HashMap<String,serde_json::Value> = serde_json::from_str(data.as_str()).unwrap();
    update("play_list",map.clone(),"id".to_string(),map.get("id").unwrap().to_string());
    "".to_string()
}

#[get("/get_play_list_content/<id>")]
async fn get_play_list_content(id:&str)->String{
serde_json::to_string(&MysqlFn::query_playlist_content(id)).unwrap()
}
#[get("/get_playlist_by_id/<id>")]
async fn get_playlist_by_id(id:&str)->String{
MysqlFn::query_playlist_by_id(id).to_string()
}
#[get("/<_..>")]
async fn everything() ->String{format!("你访问这里干什么")}
#[launch]
fn rocket() -> _ {
    let mut config = rocket::Config::default();
    config.address = IpAddr::from_str("0.0.0.0").unwrap();

    rocket::custom(config)
    .attach(cors::CORS)
    .mount("/", routes![everything])
    .mount("/api", routes![query_all_user,upload,create_user,query_user,test,update_user,get_token,download,uploadmusic,getmusiclist,getmusic,downloadmusic,getmusiclistall,new_play_list,update_play_list,get_play_list,get_play_list_content,get_playlist_by_id])
     .mount("/api", routes![get_music_page,get_pub_musics,get_pub_playlists])
}