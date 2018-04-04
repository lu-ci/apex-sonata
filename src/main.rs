#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serenity;
extern crate crossbeam_deque;
extern crate serde_yaml;
extern crate serde_json;
extern crate typemap;

pub mod sonata;
use sonata::config;
use sonata::bot;
use std::process::exit;

fn main() {
    let snt_cfg = match config::SonataConfiguration::new() {
        Ok(snt_cfg) => snt_cfg,
        Err(_) => {
            println!("Error: Couldn't create core configuration!");
            exit(1);
        }
    };
    let cfg: config::Configuration = match config::Configuration::new(snt_cfg.sigma) {
        Ok(cfg) => cfg,
        Err(_) => {
            println!("Error: Couldn't create Sigma configuration.");
            exit(1);
        }
    };
    let mut bot: bot::ApexSonata = bot::ApexSonata::new(cfg);
    println!("Starting up a Sonata instance.");
    if let Err(why) = bot.cli.start_autosharded() {
        println!("Startup Error: {:?}!", why);
    }
}
