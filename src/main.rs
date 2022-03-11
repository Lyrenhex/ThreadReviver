use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::GuildChannel;

use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn thread_update(&self, context: Context, thread: GuildChannel) {
        if let Some(metadata) = thread.thread_metadata {
            if metadata.archived {
                let result = thread
                    .id
                    .say(context.http.clone(), "Reviving archived channel...")
                    .await;
                if let Ok(msg) = result {
                    if let Err(error) = thread.id.delete_message(context.http.clone(), msg.id).await
                    {
                        eprintln!("Failed to delete revival message: {}", error);
                    }
                } else {
                    eprintln!("Failed to send revival message: {}", result.unwrap_err());
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("THREADREVIVER_DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
