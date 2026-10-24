use reqwest::Response;
use serde::{Deserialize, de::EnumAccess};
use std::collections::HashMap;
use std::fs::Metadata;
use std::{fs, vec};
use tokio::task::coop::RestoreOnPending;

const CLASSES_URL: &str =
    "https://wellfitness.perfectgym.pl/ClientPortal2/Classes/ClassCalendar/WeeklyClasses";
const CLUB_DATA_URL: &str =
    "https://wellfitness.perfectgym.pl/ClientPortal2/Clubs/Clubs/GetMembersInClubs";
const LOGIN_URL: &str = "https://wellfitness.perfectgym.pl/ClientPortal2/Auth/Login";

#[derive(Debug, Deserialize)]
struct Config {
    login: String,
    password: String,
}

async fn login(
    s: &reqwest::Client,
    login: String,
    password: String,
) -> Result<String, reqwest::Error> {
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("RememberMe", "false");
    data.insert("Login", &login);
    data.insert("Password", &password);

    let response = s.post(LOGIN_URL).json(&data).send().await?;
    response.text().await
}

async fn get_club_data(
    s: reqwest::Client,
    gyms: Vec<&str>,
) -> Result<HashMap<String, i64>, reqwest::Error> {
    let response = s.get(CLUB_DATA_URL).send().await?;

    let data: serde_json::Value = response.json().await?;

    let mut valid_data: HashMap<String, i64> = HashMap::new();

    if let Some(clubs) = data["UsersInClubList"].as_array() {
        for club in clubs {
            let club_name = club["ClubName"].as_str().unwrap_or("");
            if gyms.contains(&club_name) {
                let user_count = club["UsersCountCurrentlyInClub"].as_i64().unwrap_or(0);

                valid_data.insert(club_name.to_string(), user_count);
            }
        }
    }

    Ok(valid_data)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let gyms = vec!["Bełchatów, Kościuszki"];
    let s = reqwest::Client::builder().cookie_store(true).build()?;

    let contents = fs::read_to_string("data/config.toml").expect("Failed to read config.toml");

    let config: Config = toml::from_str(&contents).expect("Failed to read config.toml");

    login(&s, config.login, config.password).await?;

    let data = get_club_data(s, gyms).await?;

    println!("{:#?}", data);

    Ok(())
}
