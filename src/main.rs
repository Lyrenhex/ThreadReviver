use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::http::Http;
use serenity::model::channel::{ChannelType, GuildChannel};
use serenity::model::guild::Guild;

use std::collections::HashMap;
use std::env;

struct ChannelError {
    public: bool,
    channel: String,
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Revive any threads which were archived whilst the bot was not listening.
    async fn guild_create(&self, context: Context, guild: Guild, _is_new: bool) {
        let mut channel_errors: HashMap<String, Vec<ChannelError>> = HashMap::new();
        for (channel_id, channel) in guild.channels {
            if channel.kind == ChannelType::Text {
                match channel_id
                    .get_archived_private_threads(context.http.clone(), None, None)
                    .await
                {
                    Ok(threads_data) => {
                        for thread in threads_data.threads {
                            revive_thread(context.http.clone(), thread).await;
                        }
                    }
                    Err(error) => {
                        let vector = channel_errors.entry(error.to_string()).or_insert(vec![]);
                        vector.push(ChannelError {
                            public: false,
                            channel: channel.name.clone(),
                        });
                    }
                };
                match channel_id
                    .get_archived_public_threads(context.http.clone(), None, None)
                    .await
                {
                    Ok(threads_data) => {
                        for thread in threads_data.threads {
                            revive_thread(context.http.clone(), thread).await;
                        }
                    }
                    Err(error) => {
                        let vector = channel_errors.entry(error.to_string()).or_insert(vec![]);
                        vector.push(ChannelError {
                            public: true,
                            channel: channel.name,
                        });
                    }
                };
            }
        }
        // print any errors we encountered in a reasonably nicely formatted way, to help with diagnosing either actual code issues or Discord permission issues.
        if channel_errors.len() > 0 {
            eprintln!("Errors retrieving threads in guild {}", guild.name);
            for error in channel_errors.keys() {
                eprintln!("\t{}:", error);
                for channel_error in channel_errors.get(error).unwrap() {
                    eprintln!(
                        "\t\t{}\t{}",
                        if channel_error.public {
                            "public"
                        } else {
                            "private"
                        },
                        channel_error.channel
                    );
                }
            }
        }
    }

    async fn thread_update(&self, context: Context, thread: GuildChannel) {
        revive_thread(context.http, thread).await;
    }
}

async fn revive_thread(http: impl AsRef<Http>, thread: GuildChannel) {
    if let Some(metadata) = thread.thread_metadata {
        if metadata.archived {
            let result = thread
                .id
                .edit_thread(http, |thread| thread.archived(false))
                .await;
            match result {
                Ok(_) => (),
                Err(error) => eprintln!(
                    "Failed to revive thread (does the bot have permission?): {}",
                    error
                ),
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
