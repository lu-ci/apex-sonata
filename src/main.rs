#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serenity;
extern crate serde_yaml;

pub mod sonata;
use sonata::config;
use sonata::bot;

fn main() {
    let cfg: config::Configuration = config::Configuration::new("/home/alex/GitHub/apex-sigma-core".to_owned());
    let bot: bot::ApexSonata = bot::ApexSonata::new(cfg);
}
