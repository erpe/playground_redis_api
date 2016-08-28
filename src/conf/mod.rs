extern crate rustc_serialize;
extern crate toml_config;

use rustc_serialize::{Encodable, Decodable};
use std::path::Path;
use self::toml_config::ConfigFactory;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
    pub redis: RedisConfig
}

impl Config {
    pub fn redis_url(&self) -> String {
        self.redis.url()
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            redis: RedisConfig::default()
        }
    }
}


#[derive(RustcDecodable, RustcEncodable)]
pub struct RedisConfig {
    ip: String,
    db: u64
}

impl RedisConfig {
    fn url(&self) -> String {
        let pre = "redis://".to_string();
        let my_ip = self.ip.to_string();
        let my_db = self.db.to_string();
        let my_url = pre + &my_ip + "/" + &my_db;
        my_url
    }
}


impl Default for RedisConfig {
    fn default() -> RedisConfig {
        RedisConfig {
            ip: "127.0.0.1".to_owned(),
            db: 1
        }
     }
}


pub fn load() -> Config {
    let config: Config = ConfigFactory::load(Path::new("config.toml"));
    config
}
