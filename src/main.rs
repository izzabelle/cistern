mod commands;
mod config;
mod handler;
mod message_buffer;

use lazy_static::lazy_static;
use message_buffer::MessageBuffer;
use regex::Regex;
use serenity::{client::Client, framework::standard::StandardFramework};
use std::sync::Mutex;
use std::time::Instant;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "cistern bot")]
struct Opt {
    /// runs bot in production mode, defaults to development
    #[structopt(short = "p", long = "prod")]
    prod: bool,
    /// create new message buffer with given length
    #[structopt(long = "buffer")]
    buf_length: Option<usize>,
}

lazy_static! {
    pub static ref CONFIG: Mutex<config::Config> = Mutex::new(config::Config::load());
    pub static ref TIME: Instant = Instant::now();
    pub static ref OWO_REGEX: Regex =
        match Regex::new(r"(?i)[unqo0@<>~^ŪÛÕÔ][vwωw][unqo0@<>~^ŪÛÕÔ](?i)") {
            Ok(r) => r,
            Err(e) => panic!("Error occurred: {:?}", e),
        };
    pub static ref MESSAGE_BUFFER: Mutex<MessageBuffer> = match MessageBuffer::load() {
        Ok(b) => Mutex::new(b),
        Err(e) => panic!("Error occurred: {:?}", e),
    };
}

fn main() {
    let _ = TIME.elapsed();
    let opt = Opt::from_args();

    match opt.buf_length {
        Some(max_length) => {
            println!(
                "creating new message buffer with length of {}...",
                &max_length
            );
            let buf: MessageBuffer = MessageBuffer::new(max_length);
            match buf.save() {
                Ok(_) => println!("Success!"),
                Err(e) => {
                    println!("Error occurred: {:?}", e);
                    std::process::exit(1);
                }
            };
            std::process::exit(0);
        }
        None => {}
    }

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

    let mut client = Client::new(&token, handler::Handler).expect("Error creating client!");
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
