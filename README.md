# Application Filter
A CLI app that filters your /usr/share/applications by fields provided in the .desktop for each app and prints to stdout. Perfect for showing specific apps in your rofi window.


## Manual
| Flag | Description | Example |
|--|--|--|
| `--Category {category}`| Filter apps by category separated by semicolon (not case sensitive) | `--Category System;Network` |
| `--Type {type}`| Filter apps by type separated by semicolon (not case sensitive) | `--Type Application` |
| `--Keywords {keywords}`| Filter apps by keywords separated by semicolon (not case sensitive) | `--Type GPU;Radeon` |
| `--Exclude {flag}`| Remove apps that fit the flag's criteria | `--Exclude --Type GPU;Radeon` |
| `--Output {field}`| What to output for apps that meet the filter | `--Output Exec` <br> `--Output Name` |