mod message_handler;

use crate::{CONFIG, MESSAGE_BUFFER, TIME};
use serenity::{
    client::{Context, EventHandler},
    model::{
        channel::Message,
        gateway::Ready,
        guild::Member,
        id::{ChannelId, GuildId, MessageId},
        user::User,
    },
};

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _context: Context, data_about_bot: Ready) {
        CONFIG.lock().unwrap().bot_id = *data_about_bot.user.id.as_u64();
        println!(
            "{} booted successfully in {}s and is running on {} servers",
            data_about_bot.user.name,
            TIME.elapsed().as_secs(),
            data_about_bot.guilds.len()
        );
    }

    fn message(&self, _ctx: Context, message: Message) {
        if message.author.id == CONFIG.lock().unwrap().bot_id {
            return;
        }

        if message.guild_id == Some(GuildId::from(CONFIG.lock().unwrap().guild_id)) {
            MESSAGE_BUFFER.lock().unwrap().add(&message);
            match MESSAGE_BUFFER.lock().unwrap().save() {
                Ok(_) => {}
                Err(e) => panic!("Error occurred: {}", e),
            };
        }

        message_handler::generic_catch(&message);
        message_handler::furry_shit_catch(&message);
    }

    fn guild_ban_addition(&self, _ctx: Context, guild_id: GuildId, banned_user: User) {
        if guild_id == CONFIG.lock().unwrap().guild_id {
            let channel = ChannelId::from(CONFIG.lock().unwrap().bot_log_id.clone());
            let _ = channel.say(format!("`{}` has received a ban", banned_user.name));
        }
    }

    fn guild_ban_removal(&self, _ctx: Context, guild_id: GuildId, unbanned_user: User) {
        if guild_id == CONFIG.lock().unwrap().guild_id {
            let channel = ChannelId::from(CONFIG.lock().unwrap().bot_log_id.clone());
            let _ = channel.say(format!("`{}` ban has been revoked", unbanned_user.name));
        }
    }

    fn guild_member_removal(
        &self,
        _ctx: Context,
        guild_id: GuildId,
        user: User,
        _member: Option<Member>,
    ) {
        if guild_id == CONFIG.lock().unwrap().guild_id {
            let channel = ChannelId::from(CONFIG.lock().unwrap().bot_log_id.clone());
            let _ = channel.say(format!("`{}` is no longer on the server", user.name));
        }
    }
    fn message_delete(&self, _ctx: Context, channel_id: ChannelId, message_id: MessageId) {
        let message: Option<Message> = MESSAGE_BUFFER.lock().unwrap().get(message_id, channel_id);
        match message {
            Some(m) => {
                if m.guild_id == Some(GuildId::from(CONFIG.lock().unwrap().guild_id)) {
                    let channel = ChannelId::from(CONFIG.lock().unwrap().bot_log_id.clone());
                    let _ = channel.say(format!(
                        "`{}` deleted a message in `{}`:```{}```",
                        m.author.name,
                        m.channel_id.name().unwrap(),
                        m.content
                    ));
                }
            }
            None => {}
        }
    }
}
