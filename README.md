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

## How it works

Waybar spawns wttrbar process, which then stays running. Every 30 minutes (default `--interval`) wttrbar asks *wttr.in* for new data and refreshes it's output, so the indicator and tooltip are changed. So no need to use `interval` field of waybar module setting.

## Usage

- `--indicator <INDICATOR>`      Optional expression that will be shown instead of main indicator. `current_conditions` keys surrounded by `{}` can be used. Example:\
`"{ICON}{temp_C}({FeelsLikeC})"` will be transformed to "text":"🌧️0(-4)" in output **[default: "{ICON} {temp_C}"]**
- `--date-format <DATE_FORMAT>`  formats the date next to the days. see https://docs.rs/chrono/latest/chrono/format/strftime/index.html **[default: %Y-%m-%d]**
- `--location <LOCATION>`        pass a specific location to *wttr.in*
- `--hide-conditions`            show a shorter description next to each hour, like `7° Mist` instead of `7° Mist, Overcast 81%, Sunshine 17%, Frost 15%`
- `--ampm`                       display time in AM/PM format
- `--imperial`                   use imperial units instead of metric (Miles per hour, Fahrenheit). Consider changing `--indicator` to "{ICON} {temp_F}"
- `--interval <INTERVAL>`        interval of requests to *wttr.in* in minutes. Minimum is 30 **[default: 30]**
- `-h`, `--help`                 Print help
- `-V`, `--version`              Print version

e.g. `wttrbar --date-format "%m/%d" --location Paris --hide-conditions`

## Waybar configuration

Assuming `wttrbar` is in your path, it can be used like:
```json
{
  "custom/weather": {
    "format": "{} °",
    "tooltip": true,
    "exec-if": "which wttrbar",
    "exec": "wttrbar",
    "return-type": "json"
  }
}
```

## Old version

This code is based on my [old Python gist](https://gist.github.com/bjesus/f8db49e1434433f78e5200dc403d58a3) that was used for the same purpose.
