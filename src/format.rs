use chrono::prelude::*;
use serde_json::{Map, Value};
use std::collections::HashMap;

use crate::lang::Lang;
use crate::ICON_PLACEHOLDER;
use crate::*;

pub fn format_time(time: &str, ampm: bool) -> String {
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

pub fn format_temp(temp: &str) -> String {
    format!("{: >3}°", temp)
}

pub fn format_chances(hour: &serde_json::Value, lang: &Lang) -> String {
    let chances: HashMap<&str, String> = [
        ("chanceoffog", lang.fog()),
        ("chanceoffrost", lang.frost()),
        ("chanceofovercast", lang.overcast()),
        ("chanceofrain", lang.rain()),
        ("chanceofsnow", lang.snow()),
        ("chanceofsunshine", lang.sunshine()),
        ("chanceofthunder", lang.thunder()),
        ("chanceofwindy", lang.wind()),
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

pub fn format_ampm_time(day: &serde_json::Value, key: &str, ampm: bool) -> String {
    if ampm {
        day["astronomy"][0][key].as_str().unwrap().to_string()
    } else {
        NaiveTime::parse_from_str(day["astronomy"][0][key].as_str().unwrap(), "%I:%M %p")
            .unwrap()
            .format("%H:%M")
            .to_string()
    }
}

pub fn get_weather_codes(icon_family: &str) -> &'static [(i32, &'static str)] {
    match icon_family {
        "noto" => WEATHER_CODES_NOTO,
        "nerd" => WEATHER_CODES_NERD,
        _ => WEATHER_CODES_NOTO,
    }
}

pub fn format_indicator(
    weather_conditions: &Value,
    expression: String,
    icon_family: &str,
) -> String {
    if !weather_conditions.is_object() {
        return String::new();
    }
    let weather_codes = get_weather_codes(icon_family);

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
        formatted_indicator = formatted_indicator.replace(ICON_PLACEHOLDER, {
            let weather_code = weather_conditions["weatherCode"]
                .as_str()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let weather_icon = weather_codes
                .iter()
                .find(|(code, _)| *code == weather_code)
                .map(|(_, symbol)| symbol)
                .unwrap();
            weather_icon
        });
    }
    formatted_indicator
}
