use rand::{thread_rng, Rng};
use serenity::model::channel::Message;
use std::fs::read_to_string;
use std::io::Error;

pub fn generic_catch(message: &Message) {
    if message.content.contains("consume") {
        println!("{:?} consumed", message.author.name);
        let _ = message.channel_id.say("<:consume:441314166851239956>");
    }
    if message.content.contains("supreme") {
        println!("{:?} is supreme", message.author.name);
        let _ = message.channel_id.say("https://i.imgur.com/IF1Bqhk.png");
    }
    if message.content.contains("pissdrinking buttcuck") {
        println!("{:?} thanks renny, very cool", message.author.name);
        let _ = message.channel_id.say("https://i.imgur.com/g2wo1Kh.png");
    }
    if message.content.to_lowercase().as_str() == "amy is" {
        println!("{:?} knows the truth", message.author.name);
        let _ = message.channel_id.say("a bottom");
    }
}

pub fn furry_shit_catch(message: &Message) {
    match crate::OWO_REGEX.captures(message.content.as_str()) {
        Some(_) => match check_exceptions(message) {
            Ok(r) => {
                if r {
                    println!("{:?} is retarded", message.author.name);
                    match respond(&message) {
                        Ok(_) => {}
                        Err(e) => println!("An error occurred: {:?}", e),
                    }
                }
            }
            Err(e) => println!("An error occurred: {:?}", e),
        },
        None => {}
    }
}

fn check_exceptions(message: &Message) -> Result<bool, Error> {
    let json = read_to_string("./antiowo/exceptions.json")?;
    let exceptions: Vec<String> = serde_json::from_str(&json)?;

    for exception in exceptions {
        if message.content.contains(&exception) {
            return Ok(false);
        }
    }

    Ok(true)
}

fn respond(message: &Message) -> Result<(), Error> {
    let json = read_to_string("./antiowo/phrases.json")?;
    let phrases: Vec<String> = serde_json::from_str(&json)?;
    let mut rng = thread_rng();

    let _ = message.reply(&phrases[rng.gen_range(0, phrases.len() - 1)]);

    Ok(())
}
