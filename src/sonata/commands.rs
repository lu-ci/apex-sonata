use serenity::client::bridge::voice::ClientVoiceManager;
use std::sync::Arc;
use typemap::Key;
use serenity::prelude::Mutex;
use serenity::voice;

pub struct VoiceManager;

impl Key for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

command!(summon(context, message, _arguments) {
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
            let _ = message.channel_id.say(format!("Connected to {}.", vs.channel_id.unwrap().name().unwrap()));
        }
    }
});

command!(disconnect(context, message, _arguments) {
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
            let _ = message.channel_id.say(format!("Disconnected from {}.", vs.channel_id.unwrap().name().unwrap()));
        }
    }
});

command!(play(context, message, arguments) {
    let gid = message.guild_id().unwrap();
    let mut voice_manager = {
        let data = context.data.lock();
        data.get::<VoiceManager>().cloned().unwrap()
    };
    let mut manager = voice_manager.lock();
    let music_url: String = match arguments.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            let _ = message.channel_id.say("Must provide a URL to a video or audio");
            return Ok(());
        },
    };
    if let Some(handler) = manager.get_mut(gid) {
        let source = match voice::ytdl(&music_url) {
            Ok(source) => source,
            Err(_why) => {
                return Ok(());
            }
        };
        handler.play(source);
    };
});