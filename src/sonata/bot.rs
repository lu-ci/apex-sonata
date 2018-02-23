use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use config::Configuration;

struct Handler;

impl EventHandler for Handler {}

pub struct ApexSonata {
    pub client: Client,
}

impl ApexSonata {
    pub fn new(cfg: Configuration) -> ApexSonata {
        let mut client = Client::new(&cfg.dsc.get_token(), Handler).expect("Failed creating a bot client.");
        client.with_framework(StandardFramework::new());
        ApexSonata {client: client}
    }
}