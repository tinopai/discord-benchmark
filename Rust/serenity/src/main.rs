use std::env;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    // On ready handler
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    // Message hadler
    async fn message(&self, ctx: Context, msg: Message) {
        println!("Received message: '{}'", msg.content)
    }
}

#[tokio::main]
async fn main() {
    // Print the PID
    println!("Application PID: {}", std::process::id());

    // Set the token
    let token = env::var("DISCORD_TOKEN")
    .expect("Expected an enviroment token");

    // Create the client
    let mut client = Client::new(&token)
    .event_handler(Handler)
    .await
    .expect("Error creating client.");

    // If error, print error
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why)
    }
}