#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serenity;
extern crate crossbeam_deque;
extern crate serde_yaml;
extern crate serde_json;
extern crate typemap;

pub mod sonata;
use sonata::config;
use sonata::bot;

fn main() {
    let snt_cfg = config::SonataConfiguration::new();
    let cfg: config::Configuration = config::Configuration::new(snt_cfg.sigma);
    let mut bot: bot::ApexSonata = bot::ApexSonata::new(cfg);
    println!("Starting up a Sonata instance.");
    if let Err(why) = bot.cli.start_autosharded() {
        println!("Startup Error: {:?}!", why);
    }
}
