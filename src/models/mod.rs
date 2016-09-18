//mod conf;

extern crate rustc_serialize;
extern crate redis;


use rustc_serialize::json::{self, ToJson, Json};
use std::collections::BTreeMap;
use redis::Commands;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Summaries {
    pub organisations: Vec<Organisation>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Organisation {
    pub organisation_name: String,
    pub organisation_shortname: String,
    pub num_ratings: i32,
    pub num_recommendations: i32,
    pub num_confirmed: i32,
    pub average: f32,
    pub confirmed_average: f32,
    pub url: String,
    pub slug: String,
    pub updated_at: String
}

impl Organisation {

    pub fn find(id: &str, redis_url: &str) -> Organisation {
        let client = redis::Client::open(redis_url).unwrap();
        let con = client.get_connection().unwrap();
        let payload: String = con.get(id).unwrap();
        println!("payload: {}", payload);
        let orga: Organisation = json::decode(&*payload).unwrap();
        orga
    }

    pub fn dummy() -> Organisation {
        let orga = Organisation {
            organisation_name: "foobar".to_string(),
            organisation_shortname: "foo".to_string(),
            num_ratings: 42,
            num_recommendations: 42,
            num_confirmed: 41,
            average: 3.97_f32,
            confirmed_average: 3.97_f32,
            slug: "foo".to_string(),
            url: "http://localhost:3000/orgs/foo/ratings".to_string(),
            updated_at: "2016-09-01".to_string()
        };
        orga
    }
}

impl ToJson for Organisation {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("organisation_name".to_string(), self.organisation_name.to_json());
        d.insert("organisation_shortname".to_string(), self.organisation_shortname.to_json());
        d.insert("num_ratings".to_string(), self.num_ratings.to_json());
        d.insert("num_recommendations".to_string(), self.num_recommendations.to_json());
        d.insert("num_confirmed".to_string(), self.num_confirmed.to_json());
        d.insert("average".to_string(), self.average.to_json());
        d.insert("confirmed_average".to_string(), self.confirmed_average.to_json());
        d.insert("slug".to_string(), self.slug.to_json());
        d.insert("url".to_string(), self.url.to_json());
        d.insert("updated_at".to_string(), self.updated_at.to_json());
        Json::Object(d)
    }
}
