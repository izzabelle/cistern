mod commands;
mod config;
mod message_buffer;
mod message_handler;

use lazy_static::lazy_static;
use regex::Regex;
use serenity::{
    client::{Client, Context, EventHandler},
    framework::standard::StandardFramework,
    model::{
        channel::Message,
        gateway::Ready,
        guild::Member,
        id::{ChannelId, GuildId},
        user::User,
    },
};
use std::sync::Mutex;
use std::time::Instant;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "cistern bot")]
struct Opt {
    /// runs bot in production mode, defaults to development
    #[structopt(short = "p", long = "prod")]
    prod: bool,
}

struct Handler;

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
        message_handler::generic_catch(&message);
        message_handler::antiowo::furry_shit_catch(&message);
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
    /*fn message_delete(&self, _ctx: Context, channel_id: ChannelId, message_id: MessageId) {}*/
}

lazy_static! {
    pub static ref CONFIG: Mutex<config::Config> = Mutex::new(config::Config::load());
    pub static ref TIME: Instant = Instant::now();
    pub static ref OWO_REGEX: Regex =
        match Regex::new(r"(?i)[unqo0@<>~^ŪÛÕÔ][vwωw][unqo0@<>~^ŪÛÕÔ](?i)") {
            Ok(r) => r,
            Err(e) => panic!("Error occurred: {:?}", e),
        };
}

fn main() {
    let _ = TIME.elapsed();
    let opt = Opt::from_args();
    let mut token: String;
    match opt.prod {
        true => {
            println!("starting in prod...");
            token = CONFIG.lock().unwrap().prod_token.clone();
        }
        false => {
            println!("starting bot in dev...");
            token = CONFIG.lock().unwrap().dev_token.clone();
        }
    }
    let mut prefix: String;
    prefix = CONFIG.lock().unwrap().command_prefix.clone();

    let mut client = Client::new(&token, Handler).expect("Error creating client!");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix(&prefix))
            .cmd("ping", commands::ping)
            .cmd("uptime", commands::uptime)
            .cmd("owoexception", commands::exception::exception),
    );

    if let Err(e) = client.start() {
        println!("Error occurred: {:?}", e);
    }
}
