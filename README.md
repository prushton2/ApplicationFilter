# Application Filter
A CLI app that filters your /usr/share/applications by fields provided in the .desktop for each app and prints to stdout. Perfect for showing specific apps in your rofi window.


## Manual
| Flag | Description | Example |
|--|--|--|
| `--category {category}`| Filter apps by category separated by semicolon (not case sensitive) | `--category System;Network` |
| `--type {type}`| Filter apps by type separated by semicolon (not case sensitive) | `--type Application` |
| `--keywords {keywords}`| Filter apps by keywords separated by semicolon (not case sensitive) | `--type GPU;Radeon` |
| `--exclude {flag}`| Remove apps that fit the flag's criteria | `--exclude --type GPU;Radeon` |
| `--output {field}`| What to output for apps that meet the filter | `--output exec` <br> `--output Name` |
| `--stdin {field}` | search apps for the field with the value provided in stdin, and then output the entire .desktop file or a specific field provided with `--output` | `--stdin Name`<br>`--stdin exec`