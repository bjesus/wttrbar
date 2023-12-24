use std::collections::HashMap;
use std::time::Duration;

use chrono::{Local, NaiveDate, NaiveTime, Timelike};
use clap::Parser;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use retry_policies::Jitter;
use serde_json::{json, Value};

const WEATHER_CODES_DEFAULT: &[(u32, &str)] = &[
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

const WEATHER_CODES_NERD_FONT: &[(u32, &str)] = &[
    (113, "\u{f0599}"),
    (116, "\u{e302}"),
    (119, "\u{e312}"),
    (122, "\u{e311}"),
    (143, "\u{e30f}"),
    (176, "\u{e304}"),
    (179, "\u{e30b}"),
    (182, "\u{e30b}"),
    (185, "\u{e30b}"),
    (200, "\u{e208}"),
    (227, "\u{f0598}"),
    (230, "\u{f0598}"),
    (248, "\u{e30f}"),
    (260, "\u{e30f}"),
    (263, "\u{e304}"),
    (266, "\u{e304}"),
    (281, "\u{e304}"),
    (284, "\u{e304}"),
    (293, "\u{e304}"),
    (296, "\u{e304}"),
    (299, "\u{e304}"),
    (302, "\u{e304}"),
    (305, "\u{e304}"),
    (308, "\u{e304}"),
    (311, "\u{e304}"),
    (314, "\u{e304}"),
    (317, "\u{e304}"),
    (320, "\u{e30b}"),
    (323, "\u{e30b}"),
    (326, "\u{e30b}"),
    (329, "\u{e30b}"),
    (332, "\u{e30b}"),
    (335, "\u{e30b}"),
    (338, "\u{e30b}"),
    (350, "\u{e30b}"),
    (353, "\u{e304}"),
    (356, "\u{e304}"),
    (359, "\u{e304}"),
    (362, "\u{e30a}"),
    (365, "\u{e30a}"),
    (368, "\u{e30a}"),
    (371, "\u{e30a}"),
    (374, "\u{e30a}"),
    (377, "\u{e30a}"),
    (386, "\u{f0593}"),
    (389, "\u{e30a}"),
    (392, "\u{e30a}"),
    (395, "\u{e30a}"),
    (398, "\u{e30a}"),
    (401, "\u{e30a}"),
    (404, "\u{e30a}"),
    (407, "\u{e30a}"),
    (410, "\u{e30a}"),
    (413, "\u{e30a}"),
    (416, "\u{e30a}"),
    (419, "\u{e30a}"),
    (422, "\u{e30a}"),
    (425, "\u{e30a}"),
    (428, "\u{e30a}"),
    (431, "\u{e30a}"),
];

const DEFAULT_RESULT: &[(&str, &str)] = &[("text", "N/A"), ("tooltip", "N/A")];

const ICON_PLACEHOLDER: &str = "{ICON}";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        long,
        default_value = "{ICON} {temp_C}",
        help = "Optional expression that will be shown instead of main indicator. current_conditions keys surrounded by {} can be used. Example:\n\
        \"{ICON}{temp_C}({FeelsLikeC})\" will be transformed to \"text\":\"🌧️0(-4)\" in output",
        alias = "custom-indicator"
    )]
    indicator: String,

    #[arg(long, help = "Use Nerd Font icons")]
    nerd_font: bool,

    #[arg(
        long,
        default_value = "%Y-%m-%d",
        help = "Formats the date next to the days. see https://docs.rs/chrono/latest/chrono/format/strftime/index.html"
    )]
    date_format: String,

    #[arg(long, help = "pass a specific location to wttr.in")]
    location: Option<String>,

    #[arg(
        long,
        help = "Show a shorter description next to each hour, like 7° Mist instead of 7° Mist, Overcast 81%, Sunshine 17%, Frost 15%"
    )]
    hide_conditions: bool,

    #[arg(long, help = "Display time in AM/PM format")]
    ampm: bool,

    #[arg(
        long,
        alias = "fahrenheit",
        help = "Use imperial units instead of metric (Miles per hour, Fahrenheit). Consider changing `--indicator` to \"{ICON} {temp_F}\""
    )]
    imperial: bool,

    #[arg(
        long,
        default_value = "30",
        help = "Interval of requests to wttr.in in minutes. Minimum is 30"
    )]
    interval: u32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut weather_icon = "";

    let weather_url = format!(
        "https://wttr.in/{}?format=j1",
        args.location.as_ref().unwrap_or(&String::default())
    );

    let retry_policy = ExponentialBackoff::builder()
        .retry_bounds(Duration::from_secs(1), Duration::from_secs(60))
        .jitter(Jitter::Bounded)
        .base(2)
        .build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let interval = if args.interval < 30 {
        30
    } else {
        args.interval
    };

    loop {
        match get_wttr_response(&client, &weather_url).await {
            Ok(response) => {
                let parsed_response = parse_weather(response, &args, &mut weather_icon);
                println!("{}", json!(&parsed_response));
            }
            Err(_) => {
                eprintln!("Error connecting to wttr.in");
                println!("{}", json!(DEFAULT_RESULT));
            }
        }
        tokio::time::sleep(Duration::from_secs(interval as u64 * 60)).await;
    }
}

async fn get_wttr_response(
    client: &ClientWithMiddleware,
    weather_url: &String,
) -> Result<Value, reqwest_middleware::Error> {
    let response = client
        .get(weather_url)
        .send()
        .await?
        .json::<Value>()
        .await?;
    Ok(response)
}

fn find_weather_icon(code: u32, use_nerd_font: bool) -> &'static str {
    let weather_icons = if use_nerd_font {
        &WEATHER_CODES_NERD_FONT
    } else {
        &WEATHER_CODES_DEFAULT
    };

    weather_icons
        .iter()
        .find(|(c, _)| *c == code)
        .map(|(_, symbol)| *symbol)
        .unwrap_or_else(|| {
            eprintln!("Weather icon not found for code: {}", code);
            "⛅" // Default icon in case of an error
        })
}

fn parse_weather<'a>(
    weather: Value,
    args: &Args,
    weather_icon: &mut &'a str,
) -> HashMap<&'a str, String> {
    let mut data = HashMap::new();
    let current_condition = &weather["current_condition"][0];
    let feels_like = if args.imperial {
        current_condition["FeelsLikeF"].as_str().unwrap()
    } else {
        current_condition["FeelsLikeC"].as_str().unwrap()
    };

    // Extract weather description
    let _weather_desc = current_condition["weatherDesc"][0]["value"]
        .as_str()
        .unwrap_or("N/A");

    // Find the weather icon based on the weather code
    *weather_icon = find_weather_icon(
        current_condition["weatherCode"]
            .as_str()
            .unwrap()
            .parse::<u32>()
            .unwrap(),
        args.nerd_font,
    );

    // Choose the correct set of weather icons
    let weather_icons = if args.nerd_font {
        &WEATHER_CODES_NERD_FONT
    } else {
        &WEATHER_CODES_DEFAULT
    };

    // Format the text with temperature and weather condition
    let temperature_field = if args.imperial { "temp_F" } else { "temp_C" };
    let text = format!(
        "{} {}",
        *weather_icon,
        current_condition[temperature_field].as_str().unwrap()
    );
    data.insert("text", text);

    let mut tooltip_builder = String::new(); // Use StringBuilder
    let _weather_code = current_condition["weatherCode"].as_str().unwrap();
    tooltip_builder.push_str(&format!(
        "<b>{}</b> {}°\n",
        current_condition["weatherDesc"][0]["value"]
            .as_str()
            .unwrap(),
        if args.imperial {
            current_condition["temp_F"].as_str().unwrap()
        } else {
            current_condition["temp_C"].as_str().unwrap()
        },
    ));

    tooltip_builder.push_str(&format!("Feels like: {}°\n", feels_like));

    tooltip_builder.push_str(&if args.imperial {
        format!(
            "Wind: {} Mph/h\n",
            current_condition["windspeedMiles"].as_str().unwrap()
        )
    } else {
        format!(
            "Wind: {} Kmph\n",
            current_condition["windspeedKmph"].as_str().unwrap()
        )
    });

    tooltip_builder.push_str(&format!(
        "Humidity: {}%\n",
        current_condition["humidity"].as_str().unwrap()
    ));

    let nearest_area = &weather["nearest_area"][0];
    tooltip_builder.push_str(&format!(
        "Location: {}, {}, {}\n",
        nearest_area["areaName"][0]["value"].as_str().unwrap(),
        nearest_area["region"][0]["value"].as_str().unwrap(),
        nearest_area["country"][0]["value"].as_str().unwrap()
    ));

    let now = Local::now();

    let today = Local::now().date_naive();
    let mut forecast = weather["weather"].as_array().unwrap().clone();
    forecast.retain(|item| {
        let item_date =
            NaiveDate::parse_from_str(item["date"].as_str().unwrap(), "%Y-%m-%d").unwrap();
        item_date >= today
    });

    for (i, day) in forecast.iter().enumerate() {
        tooltip_builder.push_str("\n<b>");
        if i == 0 {
            tooltip_builder.push_str("Today, ");
        }
        if i == 1 {
            tooltip_builder.push_str("Tomorrow, ");
        }
        let date = NaiveDate::parse_from_str(day["date"].as_str().unwrap(), "%Y-%m-%d").unwrap();
        tooltip_builder.push_str(&format!("{}</b>\n", date.format(args.date_format.as_str())));

        if args.imperial {
            tooltip_builder.push_str(&format!(
                "{} {}° {} {}° ",
                if args.nerd_font { "\u{eaa1}" } else { "⬆️" },
                day["maxtempF"].as_str().unwrap(),
                if args.nerd_font { "\u{ea9a}" } else { "⬇️" },
                day["mintempF"].as_str().unwrap(),
            ));
        } else {
            let up_arrow_icon = if args.nerd_font { "\u{eaa1}" } else { "⬆️" };
            let down_arrow_icon = if args.nerd_font { "\u{ea9a}" } else { "⬇️" };

            tooltip_builder.push_str(&format!(
                "{} {}° {} {}° ",
                up_arrow_icon,
                day["maxtempC"].as_str().unwrap(),
                down_arrow_icon,
                day["mintempC"].as_str().unwrap(),
            ));
        }

        tooltip_builder.push_str(&format!(
            "{} {} {} {}\n",
            if args.nerd_font { "" } else { "🌅" },
            format_ampm_time(day, "sunrise", args.ampm, args.nerd_font),
            if args.nerd_font { "" } else { "🌇" },
            format_ampm_time(day, "sunset", args.ampm, args.nerd_font)
        ));

        let mut weather_icon = "";
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

            let weather_icons = if args.nerd_font {
                &WEATHER_CODES_NERD_FONT
            } else {
                &WEATHER_CODES_DEFAULT
            };

            if let Some(icon) = weather_icons
                .iter()
                .find(|(code, _)| {
                    *code
                        == hour["weatherCode"]
                            .as_str()
                            .unwrap()
                            .parse::<u32>()
                            .unwrap()
                })
                .map(|(_, symbol)| symbol)
            {
                weather_icon = icon;
            } else {
                eprintln!("Weather icon not found for code: {}", hour["weatherCode"]);
            }

            let mut tooltip_line = format!(
                "{} {} {} {}",
                format_time(hour["time"].as_str().unwrap(), args.ampm),
                weather_icon,
                if args.imperial {
                    format_temp(hour["FeelsLikeF"].as_str().unwrap())
                } else {
                    format_temp(hour["FeelsLikeC"].as_str().unwrap())
                },
                hour["weatherDesc"][0]["value"].as_str().unwrap(),
            );

            if !args.hide_conditions {
                tooltip_line.push_str(format!(", {}", format_chances(hour)).as_str());
            }

            tooltip_line.push_str("\n");
            tooltip_builder.push_str(&tooltip_line);
        }
    }

    data.insert("tooltip", tooltip_builder);

    data
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

fn format_chances(hour: &Value) -> String {
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

fn format_ampm_time(day: &Value, key: &str, ampm: bool, nerd_font: bool) -> String {
    let time = day["astronomy"][0][key].as_str().unwrap();

    let up_arrow_icon = if nerd_font { "\u{e30e}" } else { "⬆️" };
    let down_arrow_icon = if nerd_font { "\u{e302}" } else { "⬇️" };

    if ampm {
        let arrow_icon = if key == "sunrise" {
            up_arrow_icon
        } else {
            down_arrow_icon
        };
        format!("{} {}", arrow_icon, format_time(time, ampm))
    } else {
        NaiveTime::parse_from_str(time, "%I:%M %p")
            .unwrap()
            .format("%H:%M")
            .to_string()
    }
}
