use yaml_rust::{YamlLoader,Yaml};
use yaml_rust::yaml;
use std::path::PathBuf;
use std::net::SocketAddr;
use std::io::Error as IoError;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
	path:String,
	uris:Vec<SocketAddr>
}

static config_file:&'static str = "dsefs-rs.yaml";

impl Config {

	pub fn build() -> Result<Self,IoError> {
		let mut yaml_file = try!(File::open(config_file));
		let mut yaml_string = String::new();
		try!(yaml_file.read_to_string(&mut yaml_string));		
		let yaml_config = &YamlLoader::load_from_str(&mut yaml_string).unwrap()[0];
		println!("yaml_config: {:?}", yaml_config);
//		yaml_config.
		let uris = vec!();
		
		match yaml_config {
			&Yaml::Hash(hash) => {
				for key in hash.keys() {
					//println!("key: {}", key);
				}
			}
			_ => unimplemented!()	
					
		};
		Ok(Config{path:config_file.to_owned(),uris:uris})
	}
}	