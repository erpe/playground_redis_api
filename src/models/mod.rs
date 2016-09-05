extern crate rustc_serialize;

use rustc_serialize::json::{self, ToJson, Json};
use std::collections::BTreeMap;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Organisation {
    pub name: String,
    pub shortname: String,
    pub slug: String,
    pub num_ratings: i32,
    pub average: f32,
    pub updated_at: String,
}

impl Organisation {
    pub fn is_available(slug: String) -> bool {
        let looking_for = slug;
        true
    }

    pub fn find() -> Organisation {
        let orga = Organisation {
            name: "foobar".to_string(),
            shortname: "foo".to_string(),
            slug: "foo".to_string(),
            num_ratings: 42,
            average: 3.97_f32,
            updated_at: "2016-09-01".to_string(),
        };
        println!("data: {}", orga.to_json());
        orga
    }
}

impl ToJson for Organisation {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("name".to_string(), self.name.to_json());
        d.insert("shortname".to_string(), self.shortname.to_json());
        d.insert("slug".to_string(), self.slug.to_json());
        d.insert("num_ratings".to_string(), self.num_ratings.to_json());
        d.insert("average".to_string(), self.average.to_json());
        d.insert("updated_at".to_string(), self.updated_at.to_json());
        Json::Object(d)
    }
}
