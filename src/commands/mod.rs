pub mod exception;

use serenity::command;

command!(ping(_context, message) {
    println!("{:?} pinged", message.author.name);
    let _ = message.reply("Pong!");
});

command!(uptime(_context, message) {
    println!("{:?} checked uptime", message.author.name);
    let up = crate::TIME.elapsed().as_secs();
    let _ = message.channel_id.say(format!("uptime: {}s", up));
});
