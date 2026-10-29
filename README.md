## Repository info
This is a Rust rewrite of my other repository:
https://github.com/Outshader/Well-Fitness-Occupancy-meter

The tool quickly retrieves information about how many people are in specific Well Fitness gyms around Poland.

It uses the following web endpoint, which is available to anyone with valid credentials:
https://wellfitness.perfectgym.com/ClientPortal2


## Program info
### Quick start
Install Rust, then clone this repository:
`git clone https://github.com/Outshader/well-fitness-occupancy-meter_rust`

Run it with `cargo run` or build it with `cargo build`.

To run the program as a binary, build it and create the `config.toml` file. The program will not run if the file is missing or if it omits the `login` or `password` fields. The `logs.csv` file (or whichever name you choose) will be created if it doesn't exist.

The program is intended to run standalone rather than rely on tools like cron or systemd. In addition to periodically polling Well Fitness for data, it can monitor a Discord channel for commands.

The command is:
`/poil {when}`

This command lets you run a one-time poll for Well Fitness data. The `when` value can be the string `now` or a number. A number can include `m`, `h`, or `s` to specify minutes, hours, or seconds. If no unit is included, the number is treated as seconds. The poll runs when the specified duration has elapsed. You can change the command name using the `command_name` field in the [config file](#config-file). There is currently no limit as to how many custom poils you can make.


#### Command-line arguments
The program accepts the following arguments:

- `--gyms` or `-g`: Takes space- or comma-separated numbers and determines which gyms the program will retrieve data from. If omitted, gym IDs are read from the `config.toml` file in the `data` folder.
- `--csv` or `-c`: Takes no value. Determines whether the program saves results to the `logs.csv` file in the `data` folder.
- `--webhook_report` or `-w`: Takes no value. Determines whether the program sends results to the webhook defined in `config.toml`.
- `--discord_command` or `-d`: Takes no value. Determines whether the program starts the Discord bot that watches for commands.

#### Config file
The config file expects the following fields:

- `login =` string
- `password =` string
- `gyms =` [int]
- `discord =` bool
- `discord_url =` string
- `csv =` bool
- `csv_name =` string
- `discord_watcher =` bool
- `discord_token =` string
- `command_name =` string

Here:
- `bool` accepts only `true` or `false` (without quotes).
- `string` accepts text, which must be surrounded by single quotes (`'`).
- `[int]` accepts comma- or space-separated numbers enclosed in square brackets (`[]`).

Field descriptions:

- **`login`** is your Well Fitness email address.
- **`password`** is your Well Fitness password. These credentials are used to log in to the Well Fitness endpoint and retrieve gym data.
- **`gyms`** contains the IDs of the gyms you want the program to retrieve data from. Find these IDs in the `clubs.csv` file in the `data` folder.
- **`discord_url`** is the URL of the Discord channel where you want your data sent.
- **`discord`** determines whether the program sends results to the Discord webhook.
- **`discord_watcher`** determines whether to enable the Discord commands feature. It accepts only `true` or `false`.
- **`command_name`** sets the command name. The default is `poil`.
- **`discord_token`** is your Discord application token. You can obtain one from the [Discord Developer Portal](https://discord.com/developers).
- **`csv`** determines whether the results are saved to the `logs.csv` file in the `data` folder.
- **`csv_name`** is the name of the file where you want to save the logs. The default is `logs.csv`.
