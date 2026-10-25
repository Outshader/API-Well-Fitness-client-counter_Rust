use serde::{Deserialize};
use std::collections::HashMap;
use std::{fs};
use std::error::Error;

const CLUB_DATA_URL: &str =
    "https://wellfitness.perfectgym.pl/ClientPortal2/Clubs/Clubs/GetMembersInClubs";
const LOGIN_URL: &str = "https://wellfitness.perfectgym.pl/ClientPortal2/Auth/Login";

#[derive(Debug, Deserialize)]
struct Config {
    login: String,
    password: String,
    gyms: Vec<i16>,
}

async fn login(
    s: &reqwest::Client,
    login: String,
    password: String,
) -> Result<String, Box<dyn Error>> {
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("RememberMe", "false");
    data.insert("Login", &login);
    data.insert("Password", &password);

    let response = s.post(LOGIN_URL).json(&data).send().await?;
    Ok(response.text().await?)
}

async fn get_club_data(
    s: reqwest::Client,
    gyms: Vec<String>,
) -> Result<HashMap<String, i16>, Box<dyn Error>> {
    let response = s.get(CLUB_DATA_URL).send().await?;

    let data: serde_json::Value = response.json().await?;

    let mut valid_data: HashMap<String, i16> = HashMap::new();

    if let Some(clubs) = data["UsersInClubList"].as_array() {
        for club in clubs {
            let club_name = club["ClubName"].as_str().unwrap_or("");
            if gyms.iter().any(|gym| gym == club_name) {
                let user_count = match club["UsersCountCurrentlyInClub"].as_i64() {
                    Some(count) => i16::try_from(count)?,
                    None => 0,
                };

                valid_data.insert(club_name.to_string(), user_count);
            }
        }
    }

    Ok(valid_data)
}

fn get_gyms(gyms: Vec<i16>) -> Result<Vec<String>, Box<dyn Error>> {
   let mut reader = csv::ReaderBuilder::new().has_headers(true).from_path("data/clubs.csv")?;

   let mut data: HashMap<i16, (String, String)> = HashMap::new();

   for line in reader.deserialize() {
        let (k, v1, v2): (i16, String, String) = line?;
        data.insert(k, (v1, v2));
   }

   let mut valid_gyms: Vec<String> = Vec::new(); 

   for i in gyms{
       valid_gyms.push(data.get(&i).unwrap().0.clone());   
   }


   Ok(valid_gyms)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let s = reqwest::Client::builder().cookie_store(true).build()?;

    let contents = fs::read_to_string("data/config.toml").expect("Failed to read config.toml");

    let config: Config = toml::from_str(&contents).expect("Failed to read config.toml");

    let gyms = get_gyms(config.gyms)?;

    login(&s, config.login, config.password).await?;

    let data = get_club_data(s, gyms).await?;

    println!("{:#?}", data);

    Ok(())
}
