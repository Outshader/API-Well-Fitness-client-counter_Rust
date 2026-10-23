the password and mail cannot contain single quotes ('), if they do, you need to change the outermost quotes so as not to make the program interpret the quotes incorrectly.

the config now should look like so:
login = ""
password = ''


TODO(in order):
--- for now
1. JSON response iteration - finding which gyms to save the data of
2. discord webhook sending
3. saving results to CSV


--- later

4. config file additions
5. cmd line arguments
6. error hnadling