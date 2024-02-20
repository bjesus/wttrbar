use core::time;
use std::collections::HashMap;
use std::fs::{metadata, read_to_string, File};
use std::io::Write;
use std::thread;
use std::time::{Duration, SystemTime};

use chrono::prelude::*;
use clap::Parser;
use reqwest::blocking::Client;
use serde_json::{json, Map, Value};

const WEATHER_CODES: &[(i32, &str)] = &[
    (113, "☀️"),
    (116, "🌤️"),
    (119, "☁️"),
    (122, "🌥️"),
    (143, "🌫️"),
    (176, "🌦️"),
    (179, "🌧️"),
    (182, "🌨️"),
    (185, "🌨️"),
    (200, "🌩️"),
    (227, "❄️"),
    (230, "❄️"),
    (248, "🌫️"),
    (260, "🌫️"),
    (263, "🌧️"),
    (266, "🌧️"),
    (281, "🌦️"),
    (284, "🌦️"),
    (293, "🌧️"),
    (296, "🌧️"),
    (299, "🌧️"),
    (302, "🌧️"),
    (305, "🌧️"),
    (308, "🌧️"),
    (311, "🌧️"),
    (314, "🌧️"),
    (317, "🌧️"),
    (320, "🌨️"),
    (323, "🌨️"),
    (326, "🌨️"),
    (329, "🌨️"),
    (332, "🌨️"),
    (335, "🌨️"),
    (338, "🌨️"),
    (350, "🌨️"),
    (353, "🌧️"),
    (356, "🌧️"),
    (359, "🌧️"),
    (362, "🌨️"),
    (365, "🌨️"),
    (368, "🌨️"),
    (371, "🌨️"),
    (374, "🌨️"),
    (377, "🌨️"),
    (386, "🌩️"),
    (389, "🌨️"),
    (392, "🌨️"),
    (395, "🌨️"),
    (398, "🌨️"),
    (401, "🌨️"),
    (404, "🌨️"),
    (407, "🌨️"),
    (410, "🌨️"),
    (413, "🌨️"),
    (416, "🌨️"),
    (419, "🌨️"),
    (422, "🌨️"),
    (425, "🌨️"),
    (428, "🌨️"),
    (431, "🌨️"),
];

const ICON_PLACEHOLDER: &str = "{ICON}";

#[derive(Parser, Debug)]
#[command(author = "Yo'av Moshe",
version = None,
about = "A simple but detailed weather indicator for Waybar using wttr.in",
long_about = None)
]
struct Args {
    #[arg(
        long,
        default_value = "temp_C",
        help = "decide which current_conditions key will be shown on waybar"
    )]
    main_indicator: String,

    #[arg(
        long,
        help = "optional expression that will be shown instead of main indicator. current_conditions keys surrounded by {} can be used. example:\n\
        \"{ICON}{temp_C}({FeelsLikeC})\" will be transformed to \"text\":\"🌧️0(-4)\" in output"
    )]
    custom_indicator: Option<String>,

    #[arg(
        long,
        default_value = "%Y-%m-%d",
        help = "formats the date next to the days. see https://docs.rs/chrono/latest/chrono/format/strftime/index.html"
    )]
    date_format: String,

    #[arg(long, help = "pass a specific location to wttr.in")]
    location: Option<String>,

    #[arg(
        long,
        help = "shows the icon on the first line and temperature in a new line"
    )]
    vertical_view: bool,

    #[arg(
        long,
        help = " show a shorter description next to each hour, like 7° Mist instead of 7° Mist, Overcast 81%, Sunshine 17%, Frost 15%"
    )]
    hide_conditions: bool,

    #[arg(long, help = "display time in AM/PM format")]
    ampm: bool,

    #[arg(long, help = "use fahrenheit instead of celsius")]
    fahrenheit: bool,
}

fn main() {
    let args = Args::parse();

    let mut data = HashMap::new();

    let location = args.location.unwrap_or(String::new());
    let weather_url = format!("https://wttr.in/{}?format=j1", location);
    let cachefile = format!("/tmp/wttrbar-{}.json", location);

    let mut iterations = 0;
    let threshold = 20;

    let is_cache_file_recent = if let Ok(metadata) = metadata(&cachefile) {
        let ten_minutes_ago = SystemTime::now() - Duration::from_secs(600);
        metadata
            .modified()
            .map_or(false, |mod_time| mod_time > ten_minutes_ago)
    } else {
        false
    };

    let client = Client::new();
    let weather = if is_cache_file_recent {
        let json_str = read_to_string(&cachefile).unwrap();
        serde_json::from_str::<serde_json::Value>(&json_str).unwrap()
    } else {
        loop {
            match client.get(&weather_url).send() {
                Ok(response) => break response.json::<Value>().unwrap(),
                Err(_) => {
                    iterations += 1;
                    thread::sleep(time::Duration::from_millis(500 * iterations));

                    if iterations == threshold {
                        panic!("No response from endpoint!");
                    }
                }
            }
        }
    };

    if !is_cache_file_recent {
        let mut file = File::create(&cachefile)
            .expect(format!("Unable to create cache file at {}", cachefile).as_str());

        file.write_all(serde_json::to_string_pretty(&weather).unwrap().as_bytes())
            .expect(format!("Unable to write cache file at {}", cachefile).as_str());
    }
    let current_condition = &weather["current_condition"][0];
    let feels_like = current_condition[if args.fahrenheit {
        "FeelsLikeF"
    } else {
        "FeelsLikeC"
    }]
    .as_str()
    .unwrap_or(&"");
    let weather_code = current_condition["weatherCode"]
        .as_str()
        .unwrap_or(&"")
        .parse::<i32>()
        .unwrap_or(0);
    let weather_icon = WEATHER_CODES
        .iter()
        .find(|(code, _)| *code == weather_code)
        .map(|(_, symbol)| symbol)
        .unwrap_or(&"");

    let main_indicator_code = match args.main_indicator.as_str() {
        "{temp_F}" => "temp_F",
        "{temp_C}" => "temp_C",
        _ => &args.main_indicator,
    };

    let indicator = current_condition[main_indicator_code]
        .as_str()
        .unwrap_or(&"");

    let text = match args.custom_indicator {
        None => {
            if args.vertical_view {
                format!("{}\n{}", weather_icon, indicator)
            } else {
                format!("{} {}", weather_icon, indicator)
            }
        }
        Some(expression) => format_indicator(current_condition, expression, weather_icon),
    };

    data.insert("text", text);

    let mut tooltip = format!(
        "<b>{}</b> {}°\n",
        current_condition["weatherDesc"][0]["value"]
            .as_str()
            .unwrap(),
        if args.fahrenheit {
            current_condition["temp_F"].as_str().unwrap()
        } else {
            current_condition["temp_C"].as_str().unwrap()
        },
    );
    tooltip += &format!("Feels like: {}°\n", feels_like);
    tooltip += &format!(
        "Wind: {}Km/h\n",
        current_condition["windspeedKmph"].as_str().unwrap()
    );
    tooltip += &format!(
        "Humidity: {}%\n",
        current_condition["humidity"].as_str().unwrap()
    );
    let nearest_area = &weather["nearest_area"][0];
    tooltip += &format!(
        "Location: {}, {}, {}\n",
        nearest_area["areaName"][0]["value"].as_str().unwrap(),
        nearest_area["region"][0]["value"].as_str().unwrap(),
        nearest_area["country"][0]["value"].as_str().unwrap()
    );

    let now = Local::now();

    let today = Local::now().date_naive();
    let mut forecast = weather["weather"].as_array().unwrap().clone();
    forecast.retain(|item| {
        let item_date =
            NaiveDate::parse_from_str(item["date"].as_str().unwrap(), "%Y-%m-%d").unwrap();
        item_date >= today
    });

    for (i, day) in forecast.iter().enumerate() {
        tooltip += "\n<b>";
        if i == 0 {
            tooltip += "Today, ";
        }
        if i == 1 {
            tooltip += "Tomorrow, ";
        }
        let date = NaiveDate::parse_from_str(day["date"].as_str().unwrap(), "%Y-%m-%d").unwrap();
        tooltip += &format!("{}</b>\n", date.format(args.date_format.as_str()));

        if args.fahrenheit {
            tooltip += &format!(
                "⬆️ {}° ⬇️ {}° ",
                day["maxtempF"].as_str().unwrap(),
                day["mintempF"].as_str().unwrap(),
            );
        } else {
            tooltip += &format!(
                "⬆️ {}° ⬇️ {}° ",
                day["maxtempC"].as_str().unwrap(),
                day["mintempC"].as_str().unwrap(),
            );
        };

        tooltip += &format!(
            "🌅 {} 🌇 {}\n",
            format_ampm_time(day, "sunrise", args.ampm),
            format_ampm_time(day, "sunset", args.ampm),
        );
        for hour in day["hourly"].as_array().unwrap() {
            let hour_time = hour["time"].as_str().unwrap();
            let formatted_hour_time = if hour_time.len() >= 2 {
                hour_time[..hour_time.len() - 2].to_string()
            } else {
                hour_time.to_string()
            };
            if i == 0
                && now.hour() >= 2
                && formatted_hour_time.parse::<u32>().unwrap() < now.hour() - 2
            {
                continue;
            }

            let mut tooltip_line = format!(
                "{} {} {} {}",
                format_time(hour["time"].as_str().unwrap(), args.ampm),
                WEATHER_CODES
                    .iter()
                    .find(|(code, _)| *code
                        == hour["weatherCode"]
                            .as_str()
                            .unwrap()
                            .parse::<i32>()
                            .unwrap())
                    .map(|(_, symbol)| symbol)
                    .unwrap(),
                if args.fahrenheit {
                    format_temp(hour["FeelsLikeF"].as_str().unwrap())
                } else {
                    format_temp(hour["FeelsLikeC"].as_str().unwrap())
                },
                hour["weatherDesc"][0]["value"].as_str().unwrap(),
            );
            if !args.hide_conditions {
                tooltip_line += format!(", {}", format_chances(hour)).as_str();
            }
            tooltip_line += "\n";
            tooltip += &tooltip_line;
        }
    }
    data.insert("tooltip", tooltip);

    let json_data = json!(data);
    println!("{}", json_data);
}

fn format_time(time: &str, ampm: bool) -> String {
    let hour = time.replace("00", "").parse::<i32>().unwrap();

    if ampm {
        let am_or_pm = if hour >= 12 { "pm" } else { "am" };
        let hour12 = if hour == 0 || hour == 12 {
            12
        } else {
            hour % 12
        };
        format!("{: <4}", format!("{}{}", hour12, am_or_pm))
    } else {
        format!("{:02}", hour)
    }
}

fn format_temp(temp: &str) -> String {
    format!("{: >3}°", temp)
}

fn format_chances(hour: &serde_json::Value) -> String {
    let chances: HashMap<&str, &str> = [
        ("chanceoffog", "Fog"),
        ("chanceoffrost", "Frost"),
        ("chanceofovercast", "Overcast"),
        ("chanceofrain", "Rain"),
        ("chanceofsnow", "Snow"),
        ("chanceofsunshine", "Sunshine"),
        ("chanceofthunder", "Thunder"),
        ("chanceofwindy", "Wind"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut conditions = vec![];
    for (event, name) in chances.iter() {
        if let Some(chance) = hour[event].as_str() {
            if let Ok(chance_value) = chance.parse::<u32>() {
                if chance_value > 0 {
                    conditions.push((name, chance_value));
                }
            }
        }
    }
    conditions.sort_by_key(|&(_, chance_value)| std::cmp::Reverse(chance_value));
    conditions
        .iter()
        .map(|&(name, chance_value)| format!("{} {}%", name, chance_value))
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_ampm_time(day: &serde_json::Value, key: &str, ampm: bool) -> String {
    if ampm {
        day["astronomy"][0][key].as_str().unwrap().to_string()
    } else {
        NaiveTime::parse_from_str(day["astronomy"][0][key].as_str().unwrap(), "%I:%M %p")
            .unwrap()
            .format("%H:%M")
            .to_string()
    }
}
fn format_indicator(weather_conditions: &Value, expression: String, weather_icon: &&str) -> String {
    if !weather_conditions.is_object() {
        return String::new();
    }
    let default_map = Map::new();
    let weather_conditions_map = weather_conditions.as_object().unwrap_or(&default_map);
    let mut formatted_indicator = expression.to_string();
    weather_conditions_map
        .iter()
        .map(|condition| ("{".to_owned() + condition.0 + "}", condition.1))
        .for_each(|condition| {
            if formatted_indicator.contains(condition.0.as_str()) {
                let condition_value = if condition.1.is_array() {
                    condition.1.as_array().and_then(|vec| {
                        vec[0]
                            .as_object()
                            .and_then(|value_map| value_map["value"].as_str())
                    })
                } else {
                    condition.1.as_str()
                }
                .unwrap_or("");
                formatted_indicator =
                    formatted_indicator.replace(condition.0.as_str(), condition_value)
            }
        });
    if formatted_indicator.contains(ICON_PLACEHOLDER) {
        formatted_indicator = formatted_indicator.replace(ICON_PLACEHOLDER, weather_icon)
    }
    formatted_indicator
}
