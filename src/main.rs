#![deny(warnings)]

use serde_derive::{Deserialize, Serialize};

use warp::Filter;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[derive(Deserialize, Serialize)]
struct Book {
    str: String,
}

#[derive(Deserialize, Serialize)]
struct Result {
    status: u32,
    message: String,
}

fn read_txt(str: String) {
    let path = Path::new("D:/test/read.txt");
    let display = path.display();

    let mut txt = String::new();

    // 以只读方式打开路径，返回 `io::Result<File>`
    let mut open_file = match File::open(&path) {
        // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.to_string()),
        Ok(file) => file,
    };

    match open_file.read_to_string(&mut txt) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.to_string()),
        Ok(_) => print!("{}", display),
    }

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.to_string()),
        Ok(file) => file,
    };

    match file.write_all((txt + &"\n" + &str ).as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.to_string())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    };

    drop(file);

}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // POST /book
    let promote = warp::post()
        .and(warp::path("book"))
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 160))
        .and(warp::body::json())
        .map(|book: Book| {
            read_txt(book.str);
            let result = Result { status:200,message:String::from("success") };
            warp::reply::json(&result)
        });

    warp::serve(promote).run(([127, 0, 0, 1], 3030)).await
}
