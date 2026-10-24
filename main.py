from ast import parse
import json
import requests
from dotenv import load_dotenv
import os
import csv
import argparse
import re
from datetime import datetime

parser = argparse.ArgumentParser()
parser.add_argument("--gyms", "-g", nargs="+", type=str, default=None)
parser.add_argument("--csv", type=bool, default=None)
parser.add_argument("--discord", "-d", type=bool, default=None)

args = parser.parse_args()


LOGIN_URL = "https://wellfitness.perfectgym.pl/ClientPortal2/Auth/Login"
CLASSES_URL = "https://wellfitness.perfectgym.pl/ClientPortal2/Classes/ClassCalendar/WeeklyClasses"
CLUB_URL = (
    "https://wellfitness.perfectgym.pl/ClientPortal2/Clubs/Clubs/GetMembersInClubs"
)

load_dotenv("config.env")


LOGIN = os.environ.get("Login", None)
PASS = os.environ.get("Password", None)

if args.gyms:
    GYM = args.gyms
else:
    GYM = os.environ.get("Gyms", None)


### CSV
# this makes CSV be the config value if the args.csv argument is false(not given) or it's content isn't in the typical bool values (true, false)
if args.csv:
    CSV = str(args.csv)
else:
    CSV = os.environ.get("CSV", "true")

CSV = CSV.lower().strip()

if CSV not in ["true", "false"]:
    raise ValueError("CSV accepts only 'true' or 'false' as inputs. Change your config")
else:
    # takes false if csv not 'true'
    CSV = CSV == "true"

### DISCORD
if args.discord:
    DISCORD = args.discord
else:
    DISCORD = os.environ.get("Discord", "true")

DISCORD = DISCORD.strip().lower()

if DISCORD not in ["true", "false"]:
    raise ValueError(
        "Discord cmd argument and config value only accept 'true' or 'false' (or their variations) as inputs. Change your config"
    )
else:
    DISCORD = bool(DISCORD)

DISCORD_URL = os.environ.get("Discord_url", None)
if DISCORD_URL:
    DISCORD_URL = DISCORD_URL.strip()
    matches = re.fullmatch(r"^https://discord.com/api/webhooks/\d+/\w+/?$", DISCORD_URL)
    if not matches:
        raise ValueError(
            "Discord_url value doesn't look like a discord webhook link. The link must look similarly to 'https://discord.com/api/webhooks/123456789/qwerty12345'"
        )


def request_error(response: requests.Response):
    try:
        data = response.json()
        failed = 0
    except requests.exceptions.JSONDecodeError:
        failed = 1
    if response.status_code == 499:
        additional_info = "The problem might be with the authentication, make sure you're credentials in config.env are correct."
    raise RuntimeWarning(
        f"The response from {LOGIN_URL} came out with {response.status_code} \n The json response is {data if failed == 0 else 'Failed to convert response to json'}. \n {f'Some additional error information: {additional_info}' if additional_info else ''}"
    )


def login(s: requests.sessions.Session):
    data = {"RememberMe": "true", "Login": LOGIN, "Password": PASS}
    print(LOGIN, PASS)
    response = s.post(url=LOGIN_URL, json=data)
    if not response.ok:
        request_error(response)


def get_club_data(s: requests.sessions.Session, GYMS: list[str]):
    response = s.post(url=CLUB_URL)
    data = response.json()
    valid_data = {}
    for i in data["UsersInClubList"]:
        if i["ClubName"] in GYMS:
            valid_data[i["ClubName"]] = i["UsersCountCurrentlyInClub"]

    return valid_data


def get_gyms():
    GYMS = re.findall(r"\d+", GYM)
    GYMS_normal = {}
    with open("data/clubs.csv", newline="", encoding="utf-8") as file:
        lines = csv.DictReader(file)
        for i in lines:
            if i["ID"] in GYMS:
                GYMS_normal[i["ID"]] = i["club_name"]

    if len(GYMS) != len(GYMS_normal):
        missing = [i for i in GYMS if i not in GYMS_normal.keys()]
        print(f"Did not match following IDs with clubs.csv: \n{', '.join(missing)}")

    return list(GYMS_normal.values())


def save_to_CSV(data: dict[str, str]):
    fieldnames = ["GymName", "Date", "Raw ppl count"]
    mode = "r+"
    date = datetime.now().isoformat(timespec="minutes")

    if not os.path.exists("data/logs.csv"):
        mode = "a+"

    with open("data/logs.csv", mode=mode, newline="", encoding="utf-8") as file:
        writer = csv.DictWriter(file, fieldnames=fieldnames)

        if next(csv.reader(file), None) != writer.fieldnames:
            writer.writeheader()

        for k, v in data.items():
            writer.writerow({"GymName": k, "Date": date, "Raw ppl count": v})


def send_to_discord(data: dict[str, str]):
    content = "\n".join(f"{v} people at {k}" for k, v in data.items())
    if len(data) > 1:
        message = (
            f"At {datetime.now().isoformat(timespec='minutes')} logged: \n{content}"
        )
    else:
        message = f"At {datetime.now().isoformat(timespec='minutes')} logged: {content}"
    response = requests.post(url=DISCORD_URL, json={"content": message})
    if not response.ok:
        request_error(response)


def main():
    s = requests.session()
    login(s)
    GYMS = get_gyms()
    data = get_club_data(s, GYMS)
    if CSV:
        save_to_CSV(data)
    if DISCORD and DISCORD_URL:
        send_to_discord(data)


if __name__ == "__main__":
    main()
