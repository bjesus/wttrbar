<h1 align="center">
wttrbar
</h1>

<p align="center">
a simple but detailed weather indicator for <a href="https://github.com/Alexays/Waybar/">Waybar</a> using <a href="https://wttr.in/">wttr.in</a>.
</p>
<p align="center">
<img src="https://user-images.githubusercontent.com/55081/232401699-b8345fe0-ffce-4353-b51b-615389153448.png" height="400">
</p>
<hr />

## Installation

Compile yourself using `cargo build --release`, or download the precompiled binary from the [releases](https://github.com/bjesus/wttrbar/releases) page.

For Arch Linux, use the [AUR](https://aur.archlinux.org/packages/wttrbar) package.

For NixOS, use the [NixPkg](https://github.com/NixOS/nixpkgs/blob/master/pkgs/applications/misc/wttrbar/default.nix) package.

## Usage

- `--ampm` - display time in AM/PM format
- `--location STRING` - pass a specific location to wttr.in
- `--main-indicator` - decide which [`current_conditions` key](https://wttr.in/?format=j1) will be shown on waybar. defaults to `temp_C`
- `--date-format` - defaults to `%Y-%m-%d`, formats the date next to the days. see [reference](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)
- `--hide-conditions` - show a shorter descrpition next to each hour, like `7° Mist` instead of `7° Mist, Overcast 81%, Sunshine 17%, Frost 15%`
- `--fahrenheit` - use fahrenheit instead of celsius
- `--vertical-view` - shows the icon on the first line and temperature in a new line
- `--custom-indicator STRING` - optional expression that will be shown instead of main indicator. current_conditions keys surrounded by {} can be used. For example, `"{ICON}{temp_C}({FeelsLikeC})"` will be transformed to `"text":"🌧️0(-4)"` in the output
- `--lang LANG` - set language (currently `en`, `de`, `pl`, `tr`, `fr`, `ru` or `be`; submit a PR to add yours)
- `--mph` - change wind to use MPH instead of KMH

e.g. `wttrbar --date-format "%m/%d" --location Paris --hide-conditions`

### Icons

To display the weather icons correctly, you will need to have a font that supports emojis installed. The screenshot uses [Noto Emoji](https://github.com/googlefonts/noto-emoji), but you can use [other fonts](https://wiki.archlinux.org/title/fonts#Emoji_and_symbols) too.

## Waybar configuration

Assuming `wttrbar` is in your path, it can be used like:

```json
"custom/weather": {
    "format": "{}°",
    "tooltip": true,
    "interval": 3600,
    "exec": "wttrbar",
    "return-type": "json"
},
```

## Old version

This code is based on my [old Python gist](https://gist.github.com/bjesus/f8db49e1434433f78e5200dc403d58a3) that was used for the same purpose.
