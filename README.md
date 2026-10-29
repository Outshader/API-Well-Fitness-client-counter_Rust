## Repo Info
This is a Rust rewrite of my other repository 
https://github.com/Outshader/Well-Fitness-Occupancy-meter

The tool is made for quickly pulling the information about how many people are in specific Well Fitness gyms around Poland.

It uses their web endpoint available to everyone that has valid credentials to their site:
https://wellfitness.perfectgym.com/ClientPortal2


## Program Info
### Quick start
Install rust
Clone this repo
`git clone https://github.com/Outshader/well-fitness-occupancy-meter_rust`

Run it with `cargo run` or build it with `cargo build`

If you want to use the program with a binary. After building you'll only need to create the config.toml file. If you don't the program will not run, same if you omit fields: login and password. The logs.csv (or whatever you chose as the name) file however will just be created if it's missing.



#### Command-line arguments
The program accepts arguments 
`--gyms` || `-g` - with a value of space or , separated numbers. This variable determines which gyms the program will pull data from. If it’s not included, it will get the IDs from the config.toml file in the data folder
`--csv` || `-c` - with no values, determines whether the program will save the results to the logs.csv file in data folder 
`--webhook_report` || `-w` - same as csv,  determines if the program will send the result to the defined in config.toml webhook.
`--discord_command` || `-d` - same as csv, determines whether the program will start the discord bot that watches for commands.

#### Config file
The config file expects those fields.  
`login =` string
`password =` string
`gyms =` [int]
`discord =` bool
`discord_url =` string
`csv =` bool
`csv_name =` string
`discord_watcher =` bool
`discord_token =` string 

where:
bool - a field that accepts only inputs 'true' or 'false' (without the quotes).
string - a field that accepts text and your value needs to be surrounded in single quotes (').
[int] - a field that accepts numbers as input, the numbers need to be either comma or space separated. They also need to be enclosed in square brackets ([]).


Where the field:
**login** is your well fitness email
**password** is you well fitness password
Those credentials are used to log into the well fitness endpoint to get the gym data.

**gyms** The integers are the IDs of gyms you want the program to pull data from. Those IDs can be found in the clubs.csv file in the data folder.
**discord_url** is the url for your discord channel where you want your data sent.
**discord** determines whether the program will send the results to the aforementioned discord webhook endpoint.
**discord_watcher** determines whether to turn on the discord commands feature. Accepts only true or false
**discord_token** is for your discord application token. One can be obtained from the https://discord.com/developers site.  
**csv** determines whether the results will be saved in the logs.csv file in the data folder.
**csv_name** is the name of the file you'd like your logs to be in. By default it's logs.csv. 
