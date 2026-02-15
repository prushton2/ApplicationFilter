# Application Filter
A CLI app that filters your `/usr/share/applications` by fields provided in the .desktop for each app and prints to stdout. Perfect for showing specific apps in your rofi window.


## Manual
| Flag | Description | Example |
|--|--|--|
| `--categories {categories}`| Filter apps by category separated by comma (not case sensitive) | `--categories System,Network` |
| `--type {type}`| Filter apps by type separated by comma (not case sensitive) | `--type Application` |
| `--keywords {keywords}`| Filter apps by keywords separated by comma (not case sensitive) | `--keywords GPU,Radeon` |
| `--nodisplay`| Show apps with `NoDisplay` set to `true` (hidden by default) | `--nodisplay` |
| `--exclude {filter}`| Remove apps that fit the flag's criteria | `--exclude --type GPU,Radeon` |
| `--output {field}`| What to output for apps that meet the filter | `--output exec` <br> `--output Name` |
| `--stdin {field}` | Search apps for the field with the value provided in stdin, and then output the field provided with `--output`| `--stdin Name`<br>`--stdin categories`

### Terms
* `filter` refers to a flag that filters results. This does not include `--nodisplay`
* `field` refers to one of `categories`, `type`, `keywords`, `name`, `file`, `nodisplay`, or `exec`

## Examples

Heres a command to show you a dmenu of all your apps tagged `System` and run the selected one with `rofi`
```
$ ApplicationFilter --categories system --output name | rofi -dmenu | ApplicationFilter --stdin name --output exec | sh
```
The first ApplicationFilter lists the name of all apps with `system` in their categories, then pipes it to `rofi`<br>
The second ApplicationFilter takes the chosen application, searches for the application with that name, then outputs the `Exec` field in the desktop file into `sh`

---
`--output` and `--stdin` can also work with categories, types, and keywords! Heres an example that lets you select a category, and then shows all apps in that category
```
$ printf "system\nnetwork\ngtk" | rofi -dmenu | ApplicationFilter --stdin categories --output name > /tmp/selection && cat /tmp/selection | rofi -dmenu | ApplicationFilter --stdin name --output exec | sh
```
The `--stdin categories` filter will look for all apps that match the category provided in stdin, and output the names of the apps into a temp file.<br>
Its worth mentioning that `--stdin` does NOT separate the input string on commas, so passing `system,network` will not have the desired output

---
If we for some reason want to make this command even less legible, we can add filters with the `--stdin` flag
```
$ printf "system\nnetwork\ngtk" | rofi -dmenu | ApplicationFilter --stdin categories --nodisplay --type application --output name > /tmp/selection && cat /tmp/selection | rofi -dmenu | ApplicationFilter --stdin name --output exec | sh
```
The `--nodisplay` flag will display apps with `NoDisplay=true`, and the `--type application` flag filters out desktop files that arent applications.
