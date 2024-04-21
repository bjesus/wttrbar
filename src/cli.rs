use crate::Lang;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Yo'av Moshe",
version = None,
about = "A simple but detailed weather indicator for Waybar using wttr.in",
long_about = None)
]
pub struct Args {
    #[arg(
        long,
        default_value = "temp_C",
        help = "decide which current_conditions key will be shown on waybar"
    )]
    pub main_indicator: String,

    #[arg(
        long,
        help = "optional expression that will be shown instead of main indicator. current_conditions keys surrounded by {} can be used. example:\n\
        \"{ICON}{temp_C}({FeelsLikeC})\" will be transformed to \"text\":\"🌧️0(-4)\" in output"
    )]
    pub custom_indicator: Option<String>,

    #[arg(
        long,
        default_value = "%Y-%m-%d",
        help = "formats the date next to the days. see https://docs.rs/chrono/latest/chrono/format/strftime/index.html"
    )]
    pub date_format: String,

    #[arg(long, help = "pass a specific location to wttr.in")]
    pub location: Option<String>,

    #[arg(
        long,
        help = "shows the icon on the first line and temperature in a new line"
    )]
    pub vertical_view: bool,

    #[arg(
        long,
        help = "show a shorter description next to each hour, like 7° Mist instead of 7° Mist, Overcast 81%, Sunshine 17%, Frost 15%"
    )]
    pub hide_conditions: bool,

    #[arg(long, help = "display time in AM/PM format")]
    pub ampm: bool,

    #[arg(long, help = "use fahrenheit instead of celsius")]
    pub fahrenheit: bool,

    #[arg(long, short, help = "Shows the wind speed in mph")]
    pub mph: bool,

    #[arg(value_enum, short, long, help = "language to use")]
    pub lang: Option<Lang>,
}
