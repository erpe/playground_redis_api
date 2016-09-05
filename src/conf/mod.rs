extern crate rustc_serialize;
extern crate toml_config;

use rustc_serialize::{Encodable, Decodable};
use std::path::Path;
use self::toml_config::ConfigFactory;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
    pub redis: RedisConfig,
    pub server: ServerConfig,
}

impl Config {
    pub fn redis_url(&self) -> String {
        self.redis.url()
    }

    pub fn server_url(&self) -> String {
        self.server.url()
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            redis: RedisConfig::default(),
            server: ServerConfig::default(),
        }
    }
}


#[derive(RustcDecodable, RustcEncodable)]
pub struct RedisConfig {
    ip: String,
    db: u64,
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

#[derive(RustcDecodable, RustcEncodable)]
pub struct ServerConfig {
    ip: String,
    port: u64,
}

impl ServerConfig {
    fn url(&self) -> String {
        let my_ip = self.ip.to_string();
        let my_port = self.port.to_string();
        let my_url = my_ip + ":" + &my_port;
        my_url
    }
}


impl Default for RedisConfig {
    fn default() -> RedisConfig {
        RedisConfig {
            ip: "127.0.0.1".to_owned(),
            db: 1,
        }
    }
}

impl Default for ServerConfig {
    fn default() -> ServerConfig {
        ServerConfig {
            ip: "127.0.0.1".to_owned(),
            port: 8080,
        }
    }
}

pub fn load() -> Config {
    let config: Config = ConfigFactory::load(Path::new("config.toml"));
    config
}
