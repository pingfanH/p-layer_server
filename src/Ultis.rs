use std::fs;
use std::path::Path;
use rand::Rng;
use chrono::Local;
use chrono::format::strftime::StrftimeItems;
pub fn MadeUID(length: usize) -> String {
    let alphabet: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = rand::thread_rng();
    let string: String = (0..length)
        .map(|_| {
            let idx = rng.gen::<usize>() % alphabet.len();
            alphabet[idx] as char
        })
        .collect();
    string
}
pub fn MadeToken(length: usize) -> String {
    let alphabet: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZqwertyuiopasdfghjklzxcvbnm";
    let mut rng = rand::thread_rng();
    let string: String = (0..length)
        .map(|_| {
            let idx = rng.gen::<usize>() % alphabet.len();
            alphabet[idx] as char
        })
        .collect();
    string
}
pub fn GetDate()->String{
    let local_time = Local::now();
    let ymd = "%Y年%m月%d日";
    let hms = "%H:%M:%S";
    let date=format!("{} {}",ymd,hms);
    let date=StrftimeItems::new(date.as_str());
    let date=local_time.format_with_items(date);
    date.to_string()
}
pub fn NewMusicPath(name:&str,user:&str,p:bool)->String{
    let local_time = Local::now();
    let ymd = "%Y%m%d";
    let hms = "%H%M%S";
    let date=format!("{}{}",ymd,hms);
    let date=StrftimeItems::new(date.as_str());
    let date=local_time.format_with_items(date);
    let date=date.to_string();

    //let musicid=MadeToken(10);
    let mut path:String="".to_string();
    if p{
        path=format!("global/music/{}{}{}",user,name,date);
    }
    else{
        path=format!("{}/music/{}{}",user,name,date);
    }
    path
}

pub async fn readfilenameloop(path:&str)->Vec<serde_json::Value>{
    let mut jsonlist:Vec<serde_json::Value>=vec![];
    let folder_path = Path::new(path);
    let mut lists:Vec<String>=vec![];
    // 检查文件夹是否存在
    if folder_path.is_dir() {
        // 获取文件夹内所有项的迭代器
        if let Ok(entries) = fs::read_dir(folder_path) {
            // 遍历每个项
            for entry in entries {
                if let Ok(entry) = entry {
                    // 获取项的名称
                    let entry_name = entry.file_name();

                    // 将名称转换为字符串
                    if let Some(name) = entry_name.to_str() {
                        //println!("{}", name);
                        let name=name.to_string();
                        lists.push(name);
                    }
                }
            }
        }
    } else {
        println!("文件夹不存在");
    }

    for list in lists {
        println!("list:{}",list);
        let list:Vec<_>=list.split(",").collect();
        let jsonl=serde_json::json!({
                "user":list[0],
                "name":list[1],
                "date":list[2],
                "public":str2bool(list[3]),


        });
        jsonlist.push(jsonl);

    };
    jsonlist
}

pub async fn selectfromjson(jsonlist:Vec<serde_json::Value>,select:&str,value:String)->serde_json::Value{

    //let mut selectedjson:serde_json::Value;
    for i in jsonlist{
    println!("{:?}",i);
        if i[select]==value{
            return i;
        }


    }.into()

}
pub fn str2bool(s: &str) -> Option<bool> {
    match s.to_lowercase().as_str() {
        "true" | "t" | "yes" | "y" | "1" => Some(true),
        "false" | "f" | "no" | "n" | "0" => Some(false),
        _ => None,
    }
}