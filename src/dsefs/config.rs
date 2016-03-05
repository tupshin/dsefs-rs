use yaml_rust::{YamlLoader,Yaml};
use yaml_rust::yaml;
use std::path::PathBuf;
use std::net::SocketAddr;
use std::io::Error as IoError;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
	path:String,
	uris:Vec<SocketAddr>,
	fs_name: String
}

static CONFIG_FILE:&'static str = "dsefs-rs.yaml";

impl Config {

	pub fn build() -> Result<Self,IoError> {
		let mut yaml_file = try!(File::open(CONFIG_FILE));
		let mut yaml_string = String::new();
		try!(yaml_file.read_to_string(&mut yaml_string));		
		let yaml_config = &YamlLoader::load_from_str(&mut yaml_string).unwrap().clone()[0];
		let yaml_config = yaml_config.as_hash().expect("bad config");
		let mut uris = vec!();
		let mut fs_name = String::new();
		for (k,v) in yaml_config.iter() {
			match k {
				&Yaml::String(ref key) => match key.as_ref() {
					"fs_name" => {
						fs_name = v.as_str().expect("bad_config").to_owned();
					},
					"endpoints" => {
						for uri in v.as_vec().expect("bad_config") {
							uris.push(SocketAddr::from_str(uri.as_str().expect("bad_config")).unwrap());
						}
					},
					other => panic!("other: {}", other)
				},
				_ => panic!()
			}
		
	};
		Ok(Config{path:CONFIG_FILE.to_owned(),uris:uris, fs_name:fs_name})
	}
	
	pub fn uris(&self) -> Vec<SocketAddr> {
		self.uris.clone()
	}
}	