# Application Filter
A CLI app that filters your /usr/share/applications by fields provided in the .desktop for each app and prints to stdout. Perfect for showing specific apps in your rofi window.


## Manual
| Flag | Description | Example |
|--|--|--|
| `--categories {categories}`| Filter apps by category separated by comma (not case sensitive) | `--categories System,Network` |
| `--type {type}`| Filter apps by type separated by comma (not case sensitive) | `--type Application` |
| `--keywords {keywords}`| Filter apps by keywords separated by comma (not case sensitive) | `--keywords GPU,Radeon` |
| `--exclude {filter}`| Remove apps that fit the flag's criteria | `--exclude --type GPU,Radeon` |
| `--output {field}`| What to output for apps that meet the filter | `--output exec` <br> `--output Name` |
| `--stdin {field}` | search apps for the field with the value provided in stdin, and then output the entire .desktop file or a specific field provided with `--output`. Using this argument ignores all filters | `--stdin Name`<br>`--stdin exec`

### Terms
* `field` refers to either the strings `name` or `exec`
* `filter` refers to a flag that filters applications

## Examples

Heres a command to show you a dmenu of all your apps tagged `System` and run the selected one with `rofi`
```
$ ApplicationFilter --categories system --output name | rofi -dmenu | ApplicationFilter --stdin name --output exec | sh
```
The first ApplicationFilter lists the name of all apps with `system` in their categories, then pipes it to `rofi`<br>
The second ApplicationFilter takes the chosen application, searches for the application with that name, then outputs the `Exec` field in the desktop file into `sh`