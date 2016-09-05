mod conf;
mod models;

#[macro_use]
extern crate nickel;
extern crate rustc_serialize;
extern crate redis;

use models::Organisation;
use nickel::{Nickel, JsonBody, HttpRouter, MediaType};
use redis::Commands;
use std::io::{self, Write};
use rustc_serialize::json::{self, ToJson, Json};

fn main() {
    let mut server = Nickel::new();
    let cfg = conf::load();

    let url = cfg.redis_url();
    let url_slice: &str = &*url;

    set_redis_value_test(url_slice);
    get_redis_value_test(url_slice);

    let o = Organisation::find();
    let o_json = o.to_json();
    println!("o_json: {}", o_json.to_string());
    let a_json = json::encode(&o).unwrap();
    println!("a_json: {}", a_json.to_string());
    let oo: Organisation = json::decode(&*o_json.to_string()).unwrap();


    println!("created from json: {}", oo.name);

    server.utilize(router! {
        get "/api" =>  |_req, _res| {
            "your api welcomes you!\n"
        }
        get "/api/v1/rating_stats/:orga_id" => |req, mut res| {
            let orga_id = req.param("orga_id").unwrap();
            let orga = Organisation::find();
            res.set(MediaType::Json);
            // tell, don't ask!
            if Organisation::is_available(orga_id.to_string()) {
                format!("{}\n", orga.to_json())
            } else {
                format!("no organisation found: {}\n", orga_id)
            }
        }
    });

    let s_url = cfg.server_url();
    let s_url_slice: &str = &*s_url;
    server.listen(s_url_slice);
}

fn find_organisation() -> Organisation {
    let orga = Organisation {
        name: "EF".to_string(),
        shortname: "EF".to_string(),
        slug: "ef".to_string(),
        num_ratings: 42,
        average: 3.97_f32,
        updated_at: "2016-09-01".to_string(),
    };
    orga
}

// -> redis::RedisResult<()>
fn set_redis_value_test(url: &str) {
    let client = redis::Client::open(url).unwrap();
    let con = client.get_connection().unwrap();
    let orga = find_organisation();
    let orga_json = json::encode(&orga).unwrap();
    let _: () = con.set("ef", orga_json).unwrap();
}

fn get_redis_value_test(url: &str) -> String {
    let client = redis::Client::open(url).unwrap();
    let con = client.get_connection().unwrap();
    let payload: String = con.get("ef").unwrap();
    println!("payload: {}", payload);
    let res: redis::RedisResult<(String)> = con.get("ef");
    match res {
        Ok(x) => println!("result: {}", x),
        Err(e) => println!("NO result for: {}", e),
    }
    payload
}
