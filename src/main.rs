const LOGIN_URL: &str = "https://wellfitness.perfectgym.pl/ClientPortal2/Auth/Login";
const CLASSES_URL: &str = "https://wellfitness.perfectgym.pl/ClientPortal2/Classes/ClassCalendar/WeeklyClasses";
const CLUB_URL: &str = "https://wellfitness.perfectgym.pl/ClientPortal2/Clubs/Clubs/GetMembersInClubs";

async fn login(s: &reqwest::Client) -> Result<String, reqwest::Error> {
    let response = s.get("https://miniwebtool.com/api-tester/")
        .send()
        .await?;
    response.text().await
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let s = reqwest::Client::new();
    let data = login(&s).await?;

    println!("{data}");

    Ok(())
}