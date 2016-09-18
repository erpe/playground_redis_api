mod conf;
mod models;

#[macro_use]
extern crate nickel;
extern crate rustc_serialize;
extern crate redis;
extern crate hyper;

use hyper::{Client};
use std::io::Read;
use models::{Summaries, Organisation};
use nickel::{Nickel, JsonBody, HttpRouter, MediaType};
use redis::Commands;
use std::io::{self, Write};
use rustc_serialize::json::{self, ToJson, Json};


fn main() {
    let mut server = Nickel::new();
    let cfg = conf::load();
    let api_url = cfg.source_url();
    let api_url_slice: &str = &*api_url;

    warm_redis_cache(api_url_slice, &*conf::load().redis_url());
    
    server.utilize(router! {
        get "/api" =>  |_req, _res| {
            "your api welcomes you!\n"
        }
        get "/api/v1/rating_stats/:orga_id" => |req, mut res| {
            let orga_id: &str = req.param("orga_id").unwrap();
            let orga = Organisation::find(orga_id, &*conf::load().redis_url());
            res.set(MediaType::Json);
            format!("{}\n", orga.to_json())
        }
    });

    server.listen(&*cfg.server_url());
}

fn warm_redis_cache(api_url: &str, redis_url: &str)  {
    let client = Client::new();
    let mut res = client.get(api_url).send().unwrap();
    let mut buf = String::new();
    let b = res.read_to_string(&mut buf).unwrap();
    let organisations: Vec<Organisation> = json::decode(&*buf).unwrap();
    for orga in organisations {
        push_to_redis(redis_url, orga);
    }
}

fn push_to_redis(url: &str, orga: Organisation) {
    let client = redis::Client::open(url).unwrap();
    let con = client.get_connection().unwrap();
    let orga_json = json::encode(&orga).unwrap();
    let _: () = con.set(orga.slug, orga_json).unwrap();
    println!("pushing {} to redis...", orga.organisation_name);
}

fn set_redis_value_test(url: &str) {
    let client = redis::Client::open(url).unwrap();
    let con = client.get_connection().unwrap();
    let orga = Organisation::dummy();
    let orga_json = json::encode(&orga).unwrap();
    let _: () = con.set(orga.slug, orga_json).unwrap();
}

fn get_redis_value_test(url: &str) -> String {
    let client = redis::Client::open(url).unwrap();
    let con = client.get_connection().unwrap();
    let payload: String = con.get("foo").unwrap();
    payload
}
