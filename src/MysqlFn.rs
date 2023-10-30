use std::collections::HashMap;
use std::env;
use mysql::*;
use mysql::prelude::*;
use rand::distributions::Standard;
use rocket::serde::Serialize;
use serde_json::json;

#[derive(Serialize, Debug)]
pub struct musiclist{
    user:String,
    name:String,
    date:String,
    public:bool,
    id:String
}

pub fn insert(
    content:HashMap<String,serde_json::Value>,
    table:&str
){
    dotenv::from_filename(".env").expect("Failed to load .env file");
    let url = env::var("DATABASE_URL").expect("SERVER_URL not found in .env file");
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let keys = content.keys().map(|k|{
        let str=serde_json::Value::from(k.to_string()).to_string();
        let str=str[1..str.len()-1].to_owned();
        str
        }).collect::<Vec<String>>().join(",");
    let values = content.values().map(|k| {
        let str=k.to_string();
        let mut sstr="".to_string();
        match k{
            serde_json::Value::Bool(bool)=>{
                sstr=bool.to_string()
            }
            serde_json::Value::Null =>{},
            serde_json::Value::Number(_) =>{},
            serde_json::Value::String(tstr) =>{
                //let tstr=tstr[1..tstr.len()-1].to_owned();
                sstr=format!("'{}'",tstr);
            },
            serde_json::Value::Array(_) =>{},
            serde_json::Value::Object(_) =>{},
        };
        sstr
    }).collect::<Vec<String>>().join(",");
    let text: String=format!("INSERT INTO {}({})VALUES({});",table,keys,values);
    println!("{text}");
    let _conn = conn.query_drop(text);
}

pub fn query_all(url:&str,table:&str)->Vec<(String,String,String,String,String,String,String,String,String,i32)>{
    //let url = "mysql://root:123456@localhost:3306/p_layer";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let query_all = format!(r"SELECT * FROM {}",table);

    let result: Vec<(String,String,String,String,String,String,String,String,String,i32)> = conn
        .query_map(query_all, |(user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number)| {
            (user_token,user_id, user_account, user_password, user_name,user_gender,user_age,user_info,user_sign_date,user_music_number)
        })
        .unwrap();
    
    result

}
pub fn update(
    table:&str,
    content:HashMap<String,serde_json::Value>,
    Where:String,
    Value:String,
    ){
    dotenv::from_filename(".env").expect("Failed to load .env file");
    let url = env::var("DATABASE_URL").expect("SERVER_URL not found in .env file");
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let mut user_data: Vec<String> = Vec::new();
    for (key,value) in content{
        let text=format!("UPDATE {} SET {}={} WHERE {}={} ;",table,key,value,Where,Value);
        conn.query_drop(text);
    };

}

pub fn query_music(
   name:&str,content:&str
)->serde_json::Value{
    dotenv::from_filename(".env").expect("Failed to load .env file");
    let url = env::var("DATABASE_URL").expect("SERVER_URL not found in .env file");
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let query_all = format!(r"SELECT * FROM music_list WHERE {}='{}';",name,content);
    //let query_all = format!(r"SELECT * FROM {} WHERE id >= (SELECT FLOOR( MAX(id) * RAND()) FROM {} ) ORDER BY id LIMIT {};",table,table,number);
    let music:Vec<(String,String,String,bool,String)>=conn.query_map(query_all,|(user,name,date,public,id)|{
        (user,name,date,public,id)
    }).unwrap();
    let mut music_json:Vec<serde_json::Value>=vec![];
    for (user,name,date,public,id) in music{
        music_json.push(
            serde_json::json!({
                "user":user,
                "name":name,
                "date":date,
                "public":public,
                "id":id,
            })
        )
    }
    let json_array = serde_json::Value::Array(music_json);
    json_array
}
pub fn query_music_rand(
    number:usize
)->serde_json::Value{
    dotenv::from_filename(".env").expect("Failed to load .env file");
    let url = env::var("DATABASE_URL").expect("SERVER_URL not found in .env file");
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let query_all = format!(r"SELECT * FROM music_list WHERE public = true ORDER BY RAND() LIMIT {};",number);
    //let query_all = format!(r"SELECT * FROM {} WHERE id >= (SELECT FLOOR( MAX(id) * RAND()) FROM {} ) ORDER BY id LIMIT {};",table,table,number);
    let music:Vec<(String,String,String,bool,String)>=conn.query_map(query_all,|(user,name,date,public,id)|{
        (user,name,date,public,id)
    }).unwrap();
    let mut music_json:Vec<serde_json::Value>=vec![];
    for (user,name,date,public,id) in music{
        music_json.push(
            serde_json::json!({
                "user":user,
                "name":name,
                "date":date,
                "public":public,
                "id":id,
            })
        )
    }
    let json_array = serde_json::Value::Array(music_json);
    json_array
}

pub fn query_playlist_rand(
    number:usize
)->serde_json::Value{
    dotenv::from_filename(".env").expect("Failed to load .env file");
    let url = env::var("DATABASE_URL").expect("SERVER_URL not found in .env file");
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let query_all = format!(r"SELECT * FROM play_list WHERE public = true ORDER BY RAND() LIMIT {};",number);
    //let query_all = format!(r"SELECT * FROM {} WHERE id >= (SELECT FLOOR( MAX(id) * RAND()) FROM {} ) ORDER BY id LIMIT {};",table,table,number);
    let music:Vec<(String,String,String,bool,String,String,String)>=conn.query_map(query_all,|(user,name,date,public,concent,intro,id)|{
        (user,name,date,public,concent,intro,id)
    }).unwrap();
    let mut music_json:Vec<serde_json::Value>=vec![];
    for  (user,name,date,public,concent,intro,id) in music{
        music_json.push(
            serde_json::json!({
                "user":user,
                "name":name,
                "date":date,
                "public":public,
                "id":id,
                "concent":concent,
                "intro":intro
            })
        )
    }
    let json_array = serde_json::Value::Array(music_json);
    json_array
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
        .query_map(query_all, |(user, name, date, public,id)| {
            musiclist {
                user,
                name,
                date,
                public,
                id
            }
        })
        .unwrap();
    musiclist

}
pub fn queryplaylist(user:&str,public:&str)->serde_json::Value{
    dotenv::from_filename(".env").expect("Failed to load .env file");
    let DATABASE_URL = env::var("DATABASE_URL").expect("SERVER_URL not found in .env file");
    let pool = Pool::new(DATABASE_URL).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let mut query_all = format!(r"SELECT * FROM play_list WHERE user={}",user);
    if public!="all"{
        query_all=format!("{} AND public={}",query_all,public);
    }
    let musiclist: Vec<(String,String,String,bool,String,String,String)> = conn
        .query_map(query_all, |(user, name, date, public,content,intro,id)| {
            (user, name, date, public,content,intro,id)
        })
        .unwrap();
    let mut music_json:Vec<serde_json::Value>=vec![];
    for  (user,name,date,public,concent,intro,id) in musiclist{
        music_json.push(
            serde_json::json!({
                "user":user,
                "name":name,
                "date":date,
                "public":public,
                "id":id,
                "concent":concent,
                "intro":intro
            })
        )
    }
    let json_array = serde_json::Value::Array(music_json);
    json_array
}
pub fn query_playlist_content(id:&str)->Vec<serde_json::Value>{
    dotenv::from_filename(".env").expect("Failed to load .env file");
    let DATABASE_URL = env::var("DATABASE_URL").expect("SERVER_URL not found in .env file");
    let pool = Pool::new(DATABASE_URL).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let mut query = format!(r"SELECT content FROM play_list WHERE id='{}'",id);
    let content:Vec<(String)>=conn.query_map(query,|(content)|{
        (content)
    }).unwrap();
    let content:Vec<String>=serde_json::from_str(content[0].as_str()).unwrap();
    println!("{:?}",content);
    let mut musics:Vec<serde_json::Value>=vec![];
    for music in content{
        let music=query_music("id",music.as_str());
        musics.push(music[0].clone());
    }
    musics
}
pub fn query_playlist_by_id(id:&str)->serde_json::Value{
    dotenv::from_filename(".env").expect("Failed to load .env file");
    let DATABASE_URL = env::var("DATABASE_URL").expect("SERVER_URL not found in .env file");
    let pool = Pool::new(DATABASE_URL).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let mut query = format!(r"SELECT * FROM play_list WHERE id='{}'",id);
    let musiclist: Vec<(String,String,String,bool,String,String,String)> = conn
        .query_map(query, |(user, name, date, public,content,intro,id)| {
            (user, name, date, public,content,intro,id)
        })
        .unwrap();
    let mut music_json:Vec<serde_json::Value>=vec![];
    for  (user,name,date,public,concent,intro,id) in musiclist{
        music_json.push(
            serde_json::json!({
                "user":user,
                "name":name,
                "date":date,
                "public":public,
                "id":id,
                "concent":concent,
                "intro":intro
            })
        )
    }
    music_json[0].clone()
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
