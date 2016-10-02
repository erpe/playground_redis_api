extern crate rustc_serialize;
extern crate toml_config;

use std::path::Path;
use self::toml_config::ConfigFactory;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
    pub redis: RedisConfig,
    pub server: ServerConfig,
    pub source: SourceConfig,
}

impl Config {
    pub fn redis_url(&self) -> String {
        self.redis.url()
    }

    pub fn server_url(&self) -> String {
        self.server.url()
    }

    pub fn source_url(&self) -> String {
        self.source.url()
    }

    pub fn source_user(&self) -> String {
        self.source.api_user()
    }

    pub fn source_password(&self) -> String {
        self.source.api_password()
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            redis: RedisConfig::default(),
            server: ServerConfig::default(),
            source: SourceConfig::default(),
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


#[derive(RustcDecodable, RustcEncodable)]
pub struct SourceConfig {
    api_url: String,
    api_user: String,
    api_password: String,
}

impl SourceConfig {
    fn url(&self) -> String {
        self.api_url.to_string()
    }

    fn api_user(&self) -> String {
        self.api_user.to_string()
    }

    fn api_password(&self) -> String {
        self.api_password.to_string()
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

impl Default for SourceConfig {
    fn default() -> SourceConfig {
        SourceConfig {
            api_url: "localhost:3000/api/v1/rating_summaries.json".to_owned(),
            api_user: "kompass".to_owned(),
            api_password: "secret".to_owned(),
        }
    }
}

pub fn load() -> Config {
    let config: Config = ConfigFactory::load(Path::new("config.toml"));
    config
}
