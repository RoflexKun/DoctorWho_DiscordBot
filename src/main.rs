use dotenv::dotenv;
use serde::Deserialize;
use serenity::all::CreateMessage;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::builder::CreateAttachment;
use std::{env, fs, io};
use std::fs::File;
use serde_json::from_reader;
use rand::Rng;

#[derive(Debug, Deserialize, Clone)]
struct Episode {
    title: String,
    runtime: String,
    season: String,
    episode: String,
    rating: String,
    airdate: String,
}

struct Handler;

fn verify_episodes(episode_name: &str, found: &mut bool, found_episode: &mut Episode) -> Result<(), io::Error> {
    let file = File::open("src/episodes.json").expect("Error at reading episodes.json");
    let episodes: Vec<Episode> = from_reader(file)?;
    for episode in episodes.iter() {
        if episode_name.to_lowercase() == episode.title.to_lowercase() {
            *found_episode = episode.clone();
            *found = true;
        }
    }
    Ok(())
}

fn pick_quote(quote: &mut String) -> Result<(), io::Error>
{
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..100);
    let quote_file = fs::read_to_string("src/quote.txt")?;
    let mut cnt = 0;
    for i in quote_file.lines()
    {
        if cnt == num
        {
            *quote = i.to_string();
        }
        cnt+=1;
    } 
    Ok(())
}


#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        if msg.content == "+quote"
        {
            let mut quote = String::new();
            if let Err(error) = pick_quote(&mut quote){
                println!("Error at picking a quote {:?}", error);
            }
            let mut message_to_send = String::from("*");
            message_to_send += &quote;
            message_to_send += "*";
            if let Err(why) = msg.channel_id.say(&ctx.http, message_to_send).await {
                println!("Error sending message: {:?}", why);
             }
            
        }
        let first_space = msg.content.find(" ").unwrap();
        let command = &msg.content[..first_space];
        let command_input = &msg.content[first_space + 1..];
        match command {
            "+episode:" => {
                let mut found = false;
                let mut found_episode: Episode = Episode{title: "".to_string(), runtime: "".to_string(), season: "".to_string(), episode: "".to_string(), rating: "".to_string(), airdate: "".to_string()};
                if let Err(error) = verify_episodes(command_input, &mut found, &mut found_episode) {
                    println!("Error at verifying the episodes {:?}", error);
                }
                println!("{}\n{:?}", found, found_episode);
                    if found {
                        let mut message_result = String::from("###################\n");
                        message_result += "\n";
                        message_result += "📼 ";
                        message_result += "**Title**: ";
                        message_result += &found_episode.title;
                        message_result += "\n";
                        message_result += "🕰 ";
                        message_result += "**Runtime**: ";
                        message_result += &found_episode.runtime;
                        message_result += "\n";
                        message_result += "▶️ ";
                        message_result += "**Season**: ";
                        message_result += &found_episode.season;
                        message_result += "\n";
                        message_result += "⏩ ";
                        message_result += "**Episode**: ";
                        message_result += &found_episode.episode;
                        message_result += "\n";
                        message_result += "⭐️ ";
                        message_result += "**Rating**: ";
                        message_result += &found_episode.rating;
                        message_result += "\n";
                        message_result += "📅 ";
                        message_result += "**Date of airing**: ";
                        message_result += &found_episode.airdate;
                        message_result += "\n";
                        message_result += "\n###################";
                        if let Err(why) = msg.channel_id.say(&ctx.http, message_result).await {
                                   println!("Error sending message: {:?}", why);
                                }
                        }else {
                            if let Err(why) = msg.channel_id.say(&ctx.http, "There isn't an episode neither from Doctor Who(1963) nor Doctor Who(2005) which has that title!").await {
                                println!("Error sending message: {:?}", why);
                             }
                        }
            }
            "+doctor:" => {
                match command_input {       
                    "1" => {
                        let mut photo_path = String::from("C:/Users/Razvan/Desktop/K9_Rust_Project/main/doctor_who_pictures/");
                        photo_path += &command_input;
                        photo_path += ".jpg";
                        println!("{:?}", photo_path);
                        let photo = CreateAttachment::path(photo_path).await.expect("Error at creating attachment with the photo");
                        let message = CreateMessage::new().content("");
                        if let Err(why) = msg.channel_id.send_files(&ctx.http, vec![photo], message).await {
                            println!("Error sending photo: {:?}", why);
                         }
                    }
                    _ => {if let Err(why) = msg.channel_id.say(&ctx.http, "There are only 15 doctors, 1 through 15, try again!").await {
                        println!("Error sending message: {:?}", why);
                     }}
                }
            }
            _ => {println!("ceva");}
        }
        //if msg.content == "salut" {
        //    if let Err(why) = msg.channel_id.say(&ctx.http, "salut").await {
        //        println!("Error sending message: {:?}", why);
        //    }
        //}
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is online!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Invalid Token");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
