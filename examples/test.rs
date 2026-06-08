use chrono::{Local, DateTime};

fn main() {
    let local_time: DateTime<Local> = Local::now();
    println!("本地时间: {}", local_time); // 默认格式包含时区
    println!("RFC3339格式: {}", local_time.to_rfc3339());
    println!("自定义格式: {}", local_time.format("%Y-%m-%d %H:%M:%S %Z"));
}
