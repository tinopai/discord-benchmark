use std::env;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, guild::Guild},
    prelude::*,
};

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    // On ready handler
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as '{}#{}' on {} guilds", ready.user.name, ready.user.discriminator, ready.guilds.len());
    }

    // Message hadler
    async fn message(&self, ctx: Context, msg: Message) {
        let guild = Guild::get(&ctx, msg.guild_id.unwrap()).await;
        println!("'{}'@'{}': '{}'", msg.author.name, guild.unwrap().name, msg.content);
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
    let mut client = Client::builder(token)
    .event_handler(Handler)
    .await
    .expect("Error creating client.");

    // If error, print error
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why)
    }
}