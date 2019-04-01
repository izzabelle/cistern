pub mod antiowo;

use serenity::model::channel::Message;

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
