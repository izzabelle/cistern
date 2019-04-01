use serde_derive::{Deserialize, Serialize};
use serenity::{
    model::channel::Message,
    model::id::{ChannelId, MessageId},
};
use std::io::Error;
use std::ops::{Index, IndexMut};

#[derive(Serialize, Deserialize)]
pub struct MessageBuffer {
    max_length: usize,
    buffer: Vec<Message>,
}

impl MessageBuffer {
    pub fn new(max_length: usize) -> Self {
        let buffer: Vec<Message> = Vec::new();
        MessageBuffer { max_length, buffer }
    }

    pub fn load() -> Result<Self, Error> {
        let json = std::fs::read_to_string("./message_buffer.json")?;
        let buf: MessageBuffer = serde_json::from_str(&json)?;
        Ok(buf)
    }

    pub fn save(&self) -> Result<(), Error> {
        let json = serde_json::to_string(&self)?;
        std::fs::write("./message_buffer.json", json)?;
        Ok(())
    }

    pub fn add(&mut self, message: &Message) {
        if self.buffer.len() < self.max_length {
            self.buffer.push(message.clone());
        } else {
            self.buffer.remove(0);
            self.buffer.push(message.clone());
        }
    }

    pub fn get(&self, message_id: MessageId, channel_id: ChannelId) -> Option<Message> {
        for message in &self.buffer {
            if message.id == message_id && message.channel_id == channel_id {
                return Some(message.clone());
            }
        }
        None
    }
}

impl Index<usize> for MessageBuffer {
    type Output = Message;

    fn index(&self, idx: usize) -> &Message {
        &self.buffer[idx]
    }
}

impl IndexMut<usize> for MessageBuffer {
    fn index_mut(&mut self, idx: usize) -> &mut Message {
        &mut self.buffer[idx]
    }
}
