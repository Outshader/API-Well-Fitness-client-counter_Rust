use chrono::Local;
use clap::Parser;
use csv::WriterBuilder;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::{fs, string};
use std::fs::OpenOptions;

const CLUB_DATA_URL: &str =
    "https://wellfitness.perfectgym.pl/ClientPortal2/Clubs/Clubs/GetMembersInClubs";
const LOGIN_URL: &str = "https://wellfitness.perfectgym.pl/ClientPortal2/Auth/Login";

#[derive(Parser)]
struct Args {
    #[arg(short, long, num_args = 1.., value_delimiter = ',')]
    gyms: Option<Vec<i16>>,

    #[arg(short, long, default_value_t = true)]
    csv: bool,

    #[arg(short, long, default_value_t = true)]
    discord: bool,
}

#[derive(Debug, Deserialize)]
struct Config {
    login: String,
    password: String,
    gyms: Vec<i16>,
    discord: bool,
    discord_url: String,
    csv: bool,
    csv_name: Option<String>
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

async fn send_to_discord(
    url: &str,
    data: &HashMap<String, i16>,
    datetime: &String,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let mut message = String::new();

    if data.len() > 1 {
        message.push_str("Successfully logged: \n");
        for (k, v) in data {
            message.push_str(&format!("{v} people at {k} \n"))
        }
        message.push_str(&format!("at {datetime}"));
    } else {
        let (k, v) = data.iter().next().unwrap();
        message.push_str(&format!(
            "Successfully logged: {v} people at {k} at {datetime}"
        ));
    }

    let response = client
        .post(url)
        .json(&serde_json::json!({"content": message}))
        .send()
        .await?;

    Ok(())
}

fn save_to_csv(data: &HashMap<String, i16>, datetime: String, csv_name: String) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("data/{csv_name}.csv"))?;

    let needs_headers = file.metadata()?.len() == 0;

    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);

    if needs_headers {
        writer.write_record(["ClubName", "Date", "Raw ppl count"])?;
    }

    for (location, people) in data {
        writer.write_record([location.as_str(), &datetime, &people.to_string()])?;
    }

    writer.flush()?;

    Ok(())
}

fn get_gyms(gyms: Vec<i16>) -> Result<Vec<String>, Box<dyn Error>> {
    let csv_data = include_str!("../data/clubs.csv");

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_data.as_bytes());

    let mut data: HashMap<i16, (String, String)> = HashMap::new();

    for line in reader.deserialize() {
        let (k, v1, v2): (i16, String, String) = line?;
        data.insert(k, (v1, v2));
    }

    let mut valid_gyms: Vec<String> = Vec::new();

    for i in gyms {
        valid_gyms.push(data.get(&i).unwrap().0.clone());
    }

    Ok(valid_gyms)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let s = reqwest::Client::builder().cookie_store(true).build()?;
    let args = Args::parse();

    let contents = fs::read_to_string("data/config.toml").expect("Failed to read config.toml");
    let config: Config = toml::from_str(&contents).expect("Failed to read config.toml");

    let gyms = get_gyms(args.gyms.unwrap_or(config.gyms))?;

    let datetime = Local::now().format("%Y-%m-%dT%H:%M").to_string();

    let discord = config.discord;
    let discord_url = config.discord_url;

    let csv = config.csv;

    login(&s, config.login, config.password).await?;

    let data = get_club_data(s, gyms).await?;

    if discord || args.discord {
        send_to_discord(&discord_url, &data, &datetime).await?;
    };


    if csv || args.csv {
        let csv_name = config.csv_name
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.strip_suffix(".csv").unwrap_or(&s).to_string()
            })
            .unwrap_or_else(|| "logs".to_string());
        save_to_csv(&data, datetime, csv_name)?;
    }

    println!("{:#?}", data);

    Ok(())
}
