use crate::{CONFIG, MESSAGE_BUFFER};
use serenity::{
    model::{
        channel::Message,
        id::{ChannelId, GuildId, MessageId},
        user::User,
    },
    utils::Colour,
};

pub fn banned(guild_id: GuildId, banned_user: User) {
    if guild_id == CONFIG.lock().unwrap().guild_id {
        let channel = ChannelId::from(CONFIG.lock().unwrap().bot_log_id.clone());
        let _ = channel.send_message(|msg| {
            msg.embed(|e| {
                e.title("User banned")
                    .description(format!("`{}` has received a ban", banned_user.name))
                    .colour(Colour::DARK_RED)
            })
        });
    }
}

pub fn unbanned(guild_id: GuildId, unbanned_user: User) {
    if guild_id == CONFIG.lock().unwrap().guild_id {
        let channel = ChannelId::from(CONFIG.lock().unwrap().bot_log_id.clone());
        let _ = channel.send_message(|msg| {
            msg.embed(|e| {
                e.title("User banned")
                    .description(format!("`{}` ban has been revoked", unbanned_user.name))
                    .colour(Colour::DARK_GREEN)
            })
        });
    }
}

pub fn removed(guild_id: GuildId, user: User) {
    if guild_id == CONFIG.lock().unwrap().guild_id {
        let channel = ChannelId::from(CONFIG.lock().unwrap().bot_log_id.clone());
        let _ = channel.send_message(|msg| {
            msg.embed(|e| {
                e.title("User banned")
                    .description(format!("`{}` is no longer on the server", user.name))
                    .colour(Colour::DARK_GREEN)
            })
        });
    }
}

pub fn deleted(channel_id: ChannelId, message_id: MessageId) {
    let message: Option<Message> = MESSAGE_BUFFER.lock().unwrap().get(message_id, channel_id);
    match message {
        Some(m) => {
            if m.guild_id == Some(GuildId::from(CONFIG.lock().unwrap().guild_id)) {
                let channel = ChannelId::from(CONFIG.lock().unwrap().bot_log_id.clone());

                let _ = channel.send_message(|msg| {
                    msg.embed(|e| {
                        e.title("Message deleted")
                            .description(format!(
                                "`{}` deleted a message in `{}` ```{}```",
                                m.author.name,
                                m.channel_id.name().unwrap(),
                                m.content
                            ))
                            .colour(Colour::DARK_RED)
                    })
                });
            }
        }
        None => {}
    }
}
