use serenity;
use sonata::commands;
use std::collections::HashSet;
use serenity::model::channel::Message;
use serenity::Result as SerenityResult;
use serenity::model::prelude::{User, CurrentUser};
use serenity::client::{Client, Context};
use serenity::prelude::EventHandler;
use serenity::model::gateway::Ready;
use serenity::framework::standard::StandardFramework;
use config::Configuration;
use std::sync::Arc;
use sonata::commands::VoiceManager;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, context: Context, ready: Ready) {
        context.set_game_name("Rusty Testing");
        println!("{} ready!", ready.user.name);
    }
}

pub struct ApexSonata {
    pub cli: Client,
}

impl ApexSonata {
    pub fn check_msg(result: SerenityResult<Message>) {
        if let Err(why) = result {
            println!("Error sending message: {:?}", why);
        }
    }
    pub fn user() -> CurrentUser {
        serenity::http::get_current_user().unwrap()
    }
    pub fn owner() -> User {
        serenity::http::get_current_application_info().unwrap().owner
    }
    pub fn new(cfg: Configuration) -> ApexSonata {
        let mut owners = HashSet::new();
        let mut client = Client::new(&cfg.dsc.token, Handler).expect("Failed creating a bot client.");
        owners.insert(ApexSonata::owner().id);
        client.with_framework(StandardFramework::new()
            .configure(|c| c
                .prefix(&cfg.pref.prefix)
                .allow_whitespace(false)
                .case_insensitivity(true)
                .on_mention(true)
                .ignore_bots(true)
                .owners(owners)
            )
            .help(serenity::framework::standard::help_commands::with_embeds)
            .cmd("summon", commands::summon)
            .cmd("disconnect", commands::disconnect)
            .cmd("play", commands::play)
        );
        {
            let mut data = client.data.lock();
            data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
        }
        ApexSonata {cli: client}
    }
}