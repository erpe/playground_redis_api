#[macro_use] 
extern crate nickel;
extern crate rustc_serialize;
extern crate redis;

use nickel::Nickel;
use redis::Commands;
use std::io::{self, Write};

#[derive(RustcDecodable, RustcEncodable)]

struct Organisation{
    name: String,
    shortname: String,
    slug: String
}

impl Organisation {
    fn is_available(slug: String) -> bool {
        let looking_for = slug;
        true
    }
   
    fn find() -> Organisation {
        let orga = Organisation{name: "foobar".to_string(), 
                            shortname: "foo".to_string(), 
                            slug: "foo".to_string() };
        orga
    }
}

fn main() {
    let mut server = Nickel::new();

    set_redis_value_test();

    server.utilize(router! {
        get "/api" =>  |_req, _res| {
            "your api welcomes you!\n"
        }
        get "/api/v1" => |_req, _res| {
            "you found api version 1!\n"
        }
        get "/api/v1/rating_stats/:orga_id" => |req, res| {
            let orga_id = req.param("orga_id").unwrap();
            if Organisation::is_available(orga_id.to_string()) {
                let orga = Organisation::find();
                get_redis_value_test();
                format!("your'e asking for rating stats of: {}\nwe found: {}\n", orga_id, orga.name)
            } else {
                format!("no organisation found: {}\n", orga_id)
            }
        }
    });
    server.listen("127.0.0.1:8080");
}

fn find_organisation() -> Organisation {
    let orga = Organisation{name: "foobar".to_string(), 
                            shortname: "foo".to_string(), 
                            slug: "foo".to_string() };
    orga
}

// -> redis::RedisResult<()> 
fn set_redis_value_test() {
    let client = redis::Client::open("redis://127.0.0.1/11").unwrap();
    let con = client.get_connection().unwrap();
    let _ : () = con.set("ef", 43).unwrap();
}

fn get_redis_value_test() -> i32 {
    let client = redis::Client::open("redis://127.0.0.1/11").unwrap();
    let con = client.get_connection().unwrap();
    let payload : i32 = con.get("ef").unwrap();
    println!("payload: {}", payload);
    payload
}
