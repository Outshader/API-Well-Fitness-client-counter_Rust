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
#### Command-line arguments
The program accepts arguments 
`--gyms` - with a value of space or , separated numbers. This variable determines which gyms the program will pull data from. If it’s not included, it will get the IDs from the config.toml file in the data folder
`--csv` - with no values, determines whether the program will save the results to the logs.csv file in data folder 
`--discord` - same as csv, no values, determines if the program will send the result to the defined in config.toml webhook.

#### Config file
The config file expects those fields.  
`login = “"`
`password = ''`
`gyms = []`
`discord =` 
`discord_url = “”`
`csv =` 

Where the field:
**login** is your well fitness email
**password** is you well fitness password
Those credentials are used to log into the well fitness endpoint to get the gym data.
**gyms** is a sequence of numbers in brackets separated by commas (,). The integers are the IDs of gyms you want the program to pull data from. Those IDs can be found in the clubs.csv file in the data folder.
**discord_url** is the url for your discord channel where you want your data sent.
**discord** accepets only true or false as its inputs. It determines whether the program will send the results to the aforementioned discord webhook endpoint.
**csv** is likewise a bool field (accepts only true or false). It determines whether the results will be saved in the logs.csv file in the data folder.

