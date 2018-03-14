use serenity::client::bridge::voice::ClientVoiceManager;
use std::sync::Arc;
use typemap::Key;
use serenity::prelude::Mutex;

pub struct VoiceManager;

impl Key for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

command!(summon(context, message) {
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
        }
    }
});