use serenity::client::bridge::voice::ClientVoiceManager;
use std::sync::Arc;
use typemap::Key;
use serenity::prelude::Mutex;
use sonata::music::MusicCore;

pub struct VoiceManager;

impl Key for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

command!(summon(context, message, _arguments) {
    println!("{} used the SUMMON command.", message.author.name);
    let voice_state =
        match message.guild() {
            Some(guild) => {
                guild.read().voice_states.get(&message.author.id).cloned()
            },
            None => None
        };
    let mut voice_manager = {
        let data = context.data.lock();
        data.get::<VoiceManager>().cloned().unwrap()
    };
    if let (&Some(ref vs), Some(gid)) = (&voice_state, message.guild_id()) {
        let mut manager = voice_manager.lock();
        if let Some(cid) = vs.channel_id {
            manager.join(gid, cid);
            let _ = message.channel_id.send_message(|m| m
                .embed(|e| e
                    .color(0xDD2E44)
                    .title(format!("🚩 Connected to {}", vs.channel_id.unwrap().name().unwrap()))
                )
            );
        }
    }
});

command!(disconnect(context, message, _arguments) {
    println!("{} used the DISCONNECT command.", message.author.name);
    let voice_state =
        match message.guild() {
            Some(guild) => {
                guild.read().voice_states.get(&message.author.id).cloned()
            },
            None => None
        };
    let mut voice_manager = {
        let data = context.data.lock();
        data.get::<VoiceManager>().cloned().unwrap()
    };
    if let (&Some(ref vs), Some(gid)) = (&voice_state, message.guild_id()) {
        let mut manager = voice_manager.lock();
        if let Some(_cid) = vs.channel_id {
            manager.leave(gid);
            let _ = message.channel_id.send_message(|m| m
                .embed(|e| e
                    .color(0xFFCC4D)
                    .title(format!("👋 Disconnected from {}", vs.channel_id.unwrap().name().unwrap()))
                )
            );
        }
    }
});

command!(play(context, message, arguments) {
    println!("{} used the PLAY command.", message.author.name);
    let gid = message.guild_id().unwrap();
    let mut voice_manager = {
        let data = context.data.lock();
        data.get::<VoiceManager>().cloned().unwrap()
    };
    let mut manager = voice_manager.lock();
    let music_url: String = match arguments.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            let _ = message.channel_id.say("Must provide a URL to a video or audio.");
            return Ok(());
        }
    };
    let handler = manager.get_mut(gid).unwrap();
    let mdata = MusicCore::extract_data(music_url);
    &mdata.play(handler);
    let mut field_container = Vec::new();
    let playing_title: String = "🎵 Now Playing".to_owned();
    field_container.push((playing_title, &mdata.title, true));
    let _ = message.channel_id.send_message(|m| m
        .embed(|e| e
            .color(0x3B88C3)
            .fields(field_container)
            .url(&mdata.webpage_url)
            .thumbnail(&mdata.thumbnail)
        )
    );
});
