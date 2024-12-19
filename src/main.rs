use dotenv::dotenv;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use serenity::all::CreateMessage;
use serenity::async_trait;
use serenity::builder::CreateAttachment;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use std::{env, fs, io};

#[derive(Debug, Deserialize, Clone)]
struct Episode {
    title: String,
    runtime: String,
    season: String,
    episode: String,
    rating: String,
    airdate: String,
}

#[derive(Deserialize, Serialize)]
struct Player {
    username: String,
    points: u128,
}

impl Player {
    fn new() -> Self {
        Player {
            username: String::new(),
            points: 0,
        }
    }
}

#[derive(Deserialize)]
struct Question {
    question_text: String,
    question_answer: String,
}
#[derive(Deserialize, Serialize, Clone)]
struct Answer {
    answered: bool,
    correct_answer: String,
    who_answered: String,
    question_index: i32,
    user_id: String
}

impl Answer {
    fn new() -> Self {
        Answer {
            answered: false,
            correct_answer: String::new(),
            who_answered: String::new(),
            question_index: -1,
            user_id: String::new()
        }
    }

    fn current_status(&mut self) -> Result<(), io::Error> {
        let file_answer_status =
            File::open("src/answer_status.json").expect("Error at reading answer_status.json");
        let current_answer: Answer =
            from_reader(file_answer_status).expect("Error at converting from File");
        *self = current_answer;
        Ok(())
    }

    fn change_answer(&mut self, question: &mut String) -> Result<(), io::Error> {
        let file_question =
            File::open("src/questions.json").expect("Error at reading questions.json");
        let questions_list: Vec<Question> =
            from_reader(file_question).expect("Error at converting from file");
        let mut rng = rand::thread_rng();
        let mut random_num = rng.gen_range(0..questions_list.len());
        while random_num as i32 == self.question_index {
            random_num = rng.gen_range(0..questions_list.len());
        }

        for (cnt, i) in questions_list.iter().enumerate() {
            if cnt == random_num {
                self.answered = false;
                self.correct_answer = (*i.question_answer).to_string();
                self.question_index = cnt as i32;
                *question = i.question_text.to_string();
                break;
            }
        }
        let new_answer_status =
            serde_json::to_string_pretty(&self).expect("Error at converting to String");
        let mut new_file =
            File::create("src/answer_status.json").expect("Error at creating new file");
        new_file
            .write_all(new_answer_status.as_bytes())
            .expect("Error at writing new data into file");
        Ok(())
    }

    fn question_answered(&mut self, username: String, user_id: String) -> Result<(), io::Error> {
        self.answered = true;
        self.who_answered = username;
        self.user_id = user_id;
        let temp_answer = self.clone();
        let new_answer_status =
            serde_json::to_string_pretty(&temp_answer).expect("Error at converting to String");
        let mut new_file =
            File::create("src/answer_status.json").expect("Error at creating new file");
        new_file
            .write_all(new_answer_status.as_bytes())
            .expect("Error at writing new data into file");
        Ok(())
    }
}

struct Handler;

fn verify_episodes(
    episode_name: &str,
    found: &mut bool,
    found_episode: &mut Episode,
) -> Result<(), io::Error> {
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

fn pick_quote(quote: &mut String) -> Result<(), io::Error> {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..100);
    let quote_file = fs::read_to_string("src/quote.txt")?;
    for (cnt, i) in quote_file.lines().enumerate() {
        if cnt == num {
            *quote = i.to_string();
        }
    }
    Ok(())
}

fn update_leaderboard(answer_check: &Answer) -> Result<(), io::Error> {
    let file_players = File::open("src/players.json").expect("Error at reading players.json");
    let mut leaderboard: Vec<Player> =
        from_reader(file_players).expect("Error at converting from file");
    let mut new_player = true;
    for i in leaderboard.iter_mut() {
        if answer_check.who_answered == i.username {
            i.points += 1;
            new_player = false;
        }
    }

    if new_player {
        let mut player = Player::new();
        player.username = answer_check.who_answered.clone();
        player.points = 1;
        leaderboard.push(player);
    }

    let new_leaderboard =
        serde_json::to_string_pretty(&leaderboard).expect("Error at converting to String");
    let mut new_file = File::create("src/players.json").expect("Error at creating file");
    new_file
        .write_all(new_leaderboard.as_bytes())
        .expect("Error at modifying the file");
    Ok(())
}

fn output_leaderboard() -> Result<String, io::Error> {
    let file_players = File::open("src/players.json").expect("Error at opening players.json");
    let mut leaderboard: Vec<Player> =
        serde_json::from_reader(file_players).expect("Error at converting from file!");
    leaderboard.sort_by_key(|player| std::cmp::Reverse(player.points));
    let mut message_to_send = String::new();
    for (cnt, i) in leaderboard.iter().enumerate() {
        if cnt + 1 == 1 {
            message_to_send += "🥇";
            message_to_send += " ";
            message_to_send += &i.username;
            message_to_send += ": ";
            message_to_send += &(i.points.to_string());
            message_to_send += " points!";
            message_to_send += "\n";
        } else if cnt + 1 == 2 {
            message_to_send += "🥈";
            message_to_send += " ";
            message_to_send += &i.username;
            message_to_send += ": ";
            message_to_send += &(i.points.to_string());
            message_to_send += " points!";
            message_to_send += "\n";
        } else if cnt + 1 == 3 {
            message_to_send += "🥉";
            message_to_send += " ";
            message_to_send += &i.username;
            message_to_send += ": ";
            message_to_send += &(i.points.to_string());
            message_to_send += " points!";
            message_to_send += "\n";
        } else {
            message_to_send += &(cnt + 1).to_string();
            message_to_send += ".";
            message_to_send += " ";
            message_to_send += &i.username;
            message_to_send += ": ";
            message_to_send += &(i.points.to_string());
            message_to_send += " points!";
            message_to_send += "\n";
        }
    }
    Ok(message_to_send)
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        let mut answer_check = Answer::new();
        answer_check
            .current_status()
            .expect("Error at loading current status of the answer");
        if msg.content == "+quote" {
            let mut quote = String::new();
            if let Err(error) = pick_quote(&mut quote) {
                println!("Error at picking a quote {:?}", error);
            }
            let mut message_to_send = String::from("*");
            message_to_send += &quote;
            message_to_send += "*";
            if let Err(why) = msg.channel_id.say(&ctx.http, message_to_send).await {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content == "+points" {
            let leaderboard_text =
                output_leaderboard().expect("Error at outputing the leaderboard!");
            if let Err(why) = msg.channel_id.say(&ctx.http, leaderboard_text).await {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content == answer_check.correct_answer {
            answer_check
                .question_answered(msg.author.name, msg.author.id.to_string())
                .expect("Error at modifying the file");
        }
        let first_space = msg.content.find(" ");
        let command: &str;
        let command_input: &str;
        match first_space 
        {
            Some(space_index) =>
            {
                command = &msg.content[..space_index];
                command_input = &msg.content[space_index + 1..];
            }
            None => {
                command = "";
                command_input = "";
            }
        }
        
        match command {
            "+episode:" => {
                let mut found = false;
                let mut found_episode: Episode = Episode {
                    title: "".to_string(),
                    runtime: "".to_string(),
                    season: "".to_string(),
                    episode: "".to_string(),
                    rating: "".to_string(),
                    airdate: "".to_string(),
                };
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
                        message_result += "\n*All the data is acquired from IMDd, which my creator hand-wrote, so excuse any mistakes*";
                        message_result += "\n###################";
                        if let Err(why) = msg.channel_id.say(&ctx.http, message_result).await {
                                   println!("Error sending message: {:?}", why);
                                }
                        }else if let Err(why) = msg.channel_id.say(&ctx.http, "There isn't an episode neither from Doctor Who(1963) nor Doctor Who(2005) which has that title!").await {
                                println!("Error sending message: {:?}", why);
                             }
            }
            "+doctor:" => {
                let doctor_number: i32 = match  command_input.parse::<i32>().is_ok(){
                    true =>
                    {
                        command_input.parse().unwrap()
                    }
                    false =>
                    {
                       -1
                    }
                };
                
                if (1..=15).contains(&doctor_number) {
                    let mut photo_path = String::from(
                        "C:/Users/Razvan/Desktop/K9_Rust_Project/main/doctor_who_pictures/",
                    );
                    photo_path += command_input;
                    photo_path += ".jpg";
                    println!("{:?}", photo_path);
                    let photo = CreateAttachment::path(photo_path)
                        .await
                        .expect("Error at creating attachment with the photo");
                    let message = CreateMessage::new().content("");
                    if let Err(why) = msg
                        .channel_id
                        .send_files(&ctx.http, vec![photo], message)
                        .await
                    {
                        println!("Error sending photo: {:?}", why);
                    }
                } else if let Err(why) = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        "You have entered an invalid output, try a number from 1 to 15",
                    )
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }
            _ => {}
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is online!", ready.user.name);
        tokio::spawn(async move {
            let id_u32 = env::var("DISCORD_CHANNEL")
                .expect("Invalid Channel ID")
                .parse()
                .unwrap();
            let channel_id = ChannelId::new(id_u32);
            let mut answer_check: Answer = Answer::new();
            let mut question = String::new();
            answer_check
                .change_answer(&mut question)
                .expect("Error at function change answer!");
            if let Err(why) = channel_id.say(&ctx.http, &question).await {
                println!("Error sending respons to the answer: {:?}", why);
            }
            let mut ok = true;
            loop {
                let mut cnt = 0;
                while cnt != 3600 {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    cnt += 1;
                    answer_check
                        .current_status()
                        .expect("Error at reading from file!");
                    if answer_check.answered && ok {
                        let mut congrats_message = String::from("<@");
                        congrats_message += &answer_check.user_id;
                        congrats_message +="> ";
                        congrats_message += " has answered correctly!";
                        if let Err(why) = channel_id.say(&ctx.http, congrats_message).await {
                            println!("Error sending respons to the answer: {:?}", why);
                        }
                        update_leaderboard(&answer_check)
                            .expect("Error at modifying the leaderboard");
                        ok = false;
                    }
                    if cnt == 3600 {
                        if !ok {
                            answer_check
                                .change_answer(&mut question)
                                .expect("Error at function change answer!");
                            ok = true;
                            if let Err(why) = channel_id.say(&ctx.http, &question).await {
                                println!("Error sending respons to the answer: {:?}", why);
                            }
                        }
                        cnt = 0;
                    }
                }
            }
        });
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
