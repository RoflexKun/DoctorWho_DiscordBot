use std::{fs, io};
use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
struct Episode {
    title: String,
    runtime: i32, 
    season: i32, 
    episode: i32,
}

fn print_episode() -> Result<(), io::Error>
{
    let episodes_json = fs::read_to_string("episodes.json").unwrap();
    let mut episodes: Vec<Episode> = serde_json::from_str(&episodes_json).expect("Eroare");  
    for episode in episodes.iter()
    {
        println!("");
        println!("{:?}", episode);
    }
    Ok(())
}


fn main() {
    let mut user_command = String::new();
    io::stdin().read_line(&mut user_command).expect("Error at reading user input");
    match user_command.trim(){
        "quote" => {println!("This command is quote");}
        _ => {println!("We don't know this command");}
    }
    let _ = print_episode();

}
