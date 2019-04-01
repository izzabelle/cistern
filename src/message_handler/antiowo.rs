use rand::{thread_rng, Rng};
use serenity::model::channel::Message;
use std::fs::read_to_string;
use std::io::Error;

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
