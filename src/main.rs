use std::{fs, io};
use serde::Deserialize;
use serde_json;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

#[derive(Debug, Deserialize)]
struct Episode {
    title: String,
    runtime: String, 
    season: String, 
    episode: String,
    rating: String,
    airdate: String
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot{
            return; 
        }
        if msg.content == "salut" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "salut").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }   
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn print_episode() -> Result<(), io::Error>
{
    let episodes_json = fs::read_to_string("episodes.json").unwrap();
    let episodes: Vec<Episode> = serde_json::from_str(&episodes_json).expect("Eroare");  
    for episode in episodes.iter()
    {
        println!("");
        println!("{:?}", episode);
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    //let mut user_command = String::new();
    //io::stdin().read_line(&mut user_command).expect("Error at reading user input");
    //match user_command.trim(){
    //    "quote" => {println!("This command is quote");}
    //    _ => {println!("We don't know this command");}
    //}
    //let _ = print_episode();
    let token = "MTMxMjA1Njg1NTgxMTQ2MTE3MQ.GysAoB.eYs81_l8mSg4cn-T30sxrtsqBgmU7hmHvEVcDg";
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

}
