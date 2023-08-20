use serde_json::json;

pub fn str2json(str:&str)->serde_json::Value {
    
    // 解析输入字符串
    let user: Vec<&str> = str.split(",").collect();
    
    // 创建JSON对象
    let json_obj = json!({
        "user_id": user[0],
        "user_account": user[1],
        "user_password": user[2],
        "user_name": user[3],
        "user_age": user[4],
        "user_info": user[5],
        "user_sign_date": user[6],
        "user_music_number": user[7].parse::<u32>().unwrap(),
    });
    
    // 将JSON对象打印出来
    //println!("{}", serde_json::to_string_pretty(&json_obj).unwrap());
    json_obj
}
