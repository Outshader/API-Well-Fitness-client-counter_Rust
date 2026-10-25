the password and mail cannot contain single quotes ('), if they do, you need to change the outermost quotes so as not to make the program interpret the quotes incorrectly.

the config now should look like so:
login = ""
password = ''
gyms = []


TODO(in order):
--- for now
1. saving results to CSV 
2. config file additions (discord url, send_to_discord, save_to_csv)


--- later

3. cmd line arguments
4. error hnadling


-- done 
1. JSON response iteration - finding which gyms to save the data of
2. discord webhook sending
3. config file additions (gyms)