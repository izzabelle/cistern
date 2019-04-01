use serenity::command;

command!(exception(_context, message) {
    println!("{:?} attempted to add a new exception", message.author.name);
    if message.author.id == crate::CONFIG.lock().unwrap().admin_id {
        let mut contents = message.content.replace(
            format!("{}owoexception ", &crate::CONFIG.lock().unwrap().command_prefix).as_str(),
            ""
        );
        let _ = message.channel_id.say(format!("adding `{}` to exceptions", &contents));
        match insert_exception(contents) {
            Ok(_) => {
                let _ = message.channel_id.say("Success!");
            },
            Err(e) => {
                println!("Error occurred: {:?}", e);
                let _ = message.channel_id.say("Error occurred! check bot log!");
            },
        }
    } else {
        let _ = message.reply("You are not authorized to add exceptions!");
    }
});

fn insert_exception(new_exception: String) -> Result<(), std::io::Error> {
    let mut json = std::fs::read_to_string("./antiowo/exceptions.json")?;
    let mut exceptions: Vec<String> = serde_json::from_str(&json)?;
    exceptions.push(new_exception);
    json = serde_json::to_string(&exceptions)?;
    std::fs::write("./antiowo/exceptions.json", json)?;
    Ok(())
}
