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