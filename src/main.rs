use std::collections::HashMap;
use std::fs;
use reqwest::Response;
use serde::{Deserialize, de::EnumAccess};
use tokio::task::coop::RestoreOnPending;


const CLASSES_URL: &str = "https://wellfitness.perfectgym.pl/ClientPortal2/Classes/ClassCalendar/WeeklyClasses";
const CLUB_URL: &str = "https://wellfitness.perfectgym.pl/ClientPortal2/Clubs/Clubs/GetMembersInClubs";
const LOGIN_URL: &str = "https://wellfitness.perfectgym.pl/ClientPortal2/Auth/Login";



#[derive(Debug, Deserialize)]
struct Config {
    login: String,
    password: String
}

async fn login(s: &reqwest::Client, login: String, password: String) -> Result<String, reqwest::Error> {
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("RememberMe", "false");
    data.insert("Login", &login);
    data.insert("Password", &password);


    let response = s.post(LOGIN_URL)
        .json(&data)
        .send()
        .await?;
    response.text().await
}

async fn get_club_data(s: reqwest::Client) -> Result<serde_json::Value, reqwest::Error>{
    let response = s.get(CLUB_URL)
        .send()
        .await?;


    let data: serde_json::Value = response.json().await?;
    
    Ok(data)

    }


#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let s = reqwest::Client::builder()
        .cookie_store(true)
        .build()?;

    let contents = fs::read_to_string("data/config.toml")
        .expect("Failed to read config.toml");

    let config: Config = toml::from_str(&contents)
        .expect("Failed to read config.toml");



    login(&s, config.login, config.password).await?;

    let data = get_club_data(s).await?;

    println!("{data}");

    Ok(())
}