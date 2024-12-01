use core::time;
use std::collections::HashMap;
use std::fs::{metadata, read_to_string, File};
use std::io::Write;
use std::process::exit;
use std::thread;
use std::time::{Duration, SystemTime};

use chrono::prelude::*;
use clap::Parser;
use reqwest::blocking::Client;
use serde_json::{json, Value};

use crate::cli::Args;
use crate::constants::{ICON_PLACEHOLDER, WEATHER_CODES, WEATHER_CODES_NERD};
use crate::format::{format_ampm_time, format_chances, format_indicator, format_temp, format_time};
use crate::lang::Lang;

mod cli;
mod constants;
mod format;
mod lang;

fn main() {
    let args = Args::parse();
    let lang = if let Some(lang) = args.lang {
        lang
    } else {
        Lang::EN
    };

    let mut data = HashMap::new();

    let location = args.location.unwrap_or(String::new());
    let weather_url = format!(
        "https://{}/{}?format=j1",
        lang.wttr_in_subdomain(),
        location
    );
    let cachefile = format!(
        "/tmp/wttrbar-{}-{}.json",
        location,
        lang.wttr_in_subdomain()
    );

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
                Ok(response) => match response.json::<Value>() {
                    Ok(json) => break json,
                    Err(_) => {
                        println!("{{\"text\":\"⛓️‍💥\", \"tooltip\":\"invalid wttr.in response\"}}");
                        exit(0)
                    }
                },
                Err(_) => {
                    iterations += 1;
                    thread::sleep(time::Duration::from_millis(500 * iterations));

                    if iterations == threshold {
                        println!("{{\"text\":\"⛓️‍💥\", \"tooltip\":\"cannot access wttr.in\"}}");
                        exit(0)
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
    let nearest_area = &weather["nearest_area"][0];
    let feels_like = if args.fahrenheit {
        current_condition["FeelsLikeF"].as_str().unwrap()
    } else {
        current_condition["FeelsLikeC"].as_str().unwrap()
    };
    let weather_code = current_condition["weatherCode"].as_str().unwrap();

    let weather_icon = {
        if args.nerd {
            WEATHER_CODES_NERD
        } else {
            WEATHER_CODES
        }
    }
    .iter()
    .find(|(code, _)| *code == weather_code.parse::<i32>().unwrap())
    .map(|(_, symbol)| symbol)
    .unwrap();

    let text = match args.custom_indicator {
        None => {
            let main_indicator_code = if args.fahrenheit && args.main_indicator == "temp_C" {
                "temp_F"
            } else {
                args.main_indicator.as_str()
            };
            let indicator = current_condition[main_indicator_code].as_str().unwrap();
            if args.vertical_view {
                format!("{}\n{}", weather_icon, indicator)
            } else {
                format!("{} {}", weather_icon, indicator)
            }
        }
        Some(expression) => {
            format_indicator(current_condition, nearest_area, expression, weather_icon)
        }
    };
    data.insert("text", text);

    let mut tooltip = format!(
        "<b>{}</b> {}°\n",
        current_condition[lang.weather_desc()][0]["value"]
            .as_str()
            .unwrap(),
        if args.fahrenheit {
            current_condition["temp_F"].as_str().unwrap()
        } else {
            current_condition["temp_C"].as_str().unwrap()
        },
    );
    tooltip += &format!("{}: {}°\n", lang.feels_like(), feels_like);
    if args.mph {
        tooltip += &format!(
            "{}: {} mph\n",
            lang.wind(),
            current_condition["windspeedMiles"].as_str().unwrap()
        );
    } else {
        tooltip += &format!(
            "{}: {} km/h\n",
            lang.wind(),
            current_condition["windspeedKmph"].as_str().unwrap()
        );
    }
    tooltip += &format!(
        "{}: {}%\n",
        lang.humidity(),
        current_condition["humidity"].as_str().unwrap()
    );
    let nearest_area = &weather["nearest_area"][0];
    tooltip += &format!(
        "{}: {}, {}, {}\n",
        lang.location(),
        nearest_area["areaName"][0]["value"].as_str().unwrap(),
        nearest_area["region"][0]["value"].as_str().unwrap(),
        nearest_area["country"][0]["value"].as_str().unwrap()
    );

    if args.observation_time {
        if let Some(obs_time) = current_condition["observation_time"].as_str() {
            if let Ok(time) = NaiveTime::parse_from_str(obs_time, "%I:%M %p") {
                let formatted_time = if args.ampm {
                    obs_time.to_string()
                } else {
                    time.format("%H:%M").to_string()
                };
                tooltip += &format!("{}: {}\n", lang.observation_time(), formatted_time);
            }
        }
    }

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
            tooltip += &format!("{}, ", lang.today());
        }
        if i == 1 {
            tooltip += &format!("{}, ", lang.tomorrow());
        }
        let date = NaiveDate::parse_from_str(day["date"].as_str().unwrap(), "%Y-%m-%d").unwrap();
        tooltip += &format!("{}</b>\n", date.format(args.date_format.as_str()));

        let (max_temp, min_temp) = if args.fahrenheit {
            (
                day["maxtempF"].as_str().unwrap(),
                day["mintempF"].as_str().unwrap(),
            )
        } else {
            (
                day["maxtempC"].as_str().unwrap(),
                day["mintempC"].as_str().unwrap(),
            )
        };

        tooltip += &format!(
            "{} {}° {} {}° ",
            if args.nerd { "󰳡" } else { "⬆️" },
            max_temp,
            if args.nerd { "󰳛" } else { "⬇️" },
            min_temp
        );

        tooltip += &format!(
            "{} {} {} {}\n",
            if args.nerd { "󰖜" } else { "🌅" },
            format_ampm_time(day, "sunrise", args.ampm),
            if args.nerd { "󰖛" } else { "🌇" },
            format_ampm_time(day, "sunset", args.ampm)
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
                if args.nerd {
                    WEATHER_CODES_NERD
                } else {
                    WEATHER_CODES
                }
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
                hour[lang.weather_desc()][0]["value"].as_str().unwrap(),
            );
            if !args.hide_conditions {
                tooltip_line += format!(", {}", format_chances(hour, &lang)).as_str();
            }
            tooltip_line += "\n";
            tooltip += &tooltip_line;
        }
    }
    data.insert("tooltip", tooltip);

    let css_class = current_condition[lang.weather_desc()][0]["value"]
        .as_str()
        .unwrap()
        .to_lowercase()
        .split(',')
        .next()
        .map(|s| s.trim().replace(' ', "_"))
        .unwrap_or_default();
    data.insert("class", css_class);

    let json_data = json!(data);
    println!("{}", json_data);
}
