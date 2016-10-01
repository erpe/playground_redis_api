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
use hyper::header::{Headers, Authorization, Basic};
use conf::{Config};

fn main() {
    let mut server = Nickel::new();
    let cfg = conf::load();
    let server_url = &*cfg.server_url();

    warm_redis_cache(cfg);
    
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

    server.listen(server_url);
}

fn warm_redis_cache(cfg: Config)  {
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set(
        Authorization(
            Basic {
                username: cfg.source_user(),
                password: Some(cfg.source_password())
            }
            )
        );

    let mut res = client.get(&*cfg.source_url()).headers(headers).send().unwrap();

    let mut buf = String::new();
    let redis_url = &*cfg.redis_url();
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
