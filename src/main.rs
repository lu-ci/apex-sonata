#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serenity;
extern crate serde_yaml;
extern crate typemap;

pub mod sonata;
use sonata::config;
use sonata::bot;

fn main() {
    let cfg: config::Configuration = config::Configuration::new("../apex-sigma-core".to_owned());
    let mut bot: bot::ApexSonata = bot::ApexSonata::new(cfg);
    println!("Starting up a Sonata instance.");
    if let Err(why) = bot.cli.start_autosharded() {
        println!("Startup Error: {:?}!", why);
    }
    println!("Apex Sonata Owned by {} started.", bot::ApexSonata::owner().name)
}
