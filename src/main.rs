#[macro_use] extern crate serde_derive;
extern crate serde_yaml;

pub mod sonata;
use sonata::config;

fn main() {
    let cfg: config::Configuration = config::Configuration::new("/home/alex/GitHub/apex-sigma-core".to_owned());
    println!("Token: {}", cfg.dsc.get_token());
}
