use sonata::bot::ApexSonata;
use serenity::model::id::ChannelId;
use serenity::client::CACHE;
use serenity::client::bridge::voice::ClientVoiceManager;
use std::sync::Arc;
use typemap::Key;
use serenity::prelude::Mutex;

struct VoiceManager;
impl Key for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

command!(summon(ctx, msg, args) {
    let vc_target = match args.single::<u64>() {
        Ok(id) => ChannelId(id),
        Err(_why) => {
            ApexSonata::check_msg(msg.reply("Sorry, you need to be in a Voice Channel that I can access."));
            return Ok(());
        }
    };
    let guild_id = match CACHE.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            ApexSonata::check_msg(msg.reply("Sorry, this is not something you can do in a DM."));
            return Ok(());
        },
    };
    let mut manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().unwrap();
    let mut manager = manager_lock.lock();
});