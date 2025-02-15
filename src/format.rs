use chrono::prelude::*;
use serde_json::Value;
use std::collections::HashMap;

use crate::lang::Lang;
use crate::ICON_PLACEHOLDER;

pub fn extract_hour_time(hour: &serde_json::Value) -> NaiveTime {
    let hour_int = hour.as_str().unwrap().parse::<u32>().unwrap() / 100;

    let hour_time = NaiveTime::parse_from_str(
        format!("{}:00", &hour_int).as_str(),
        "%k:%M"
    ).unwrap();
    return hour_time
}

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

pub fn extract_day_data(day: &serde_json::Value, key: &str) -> NaiveTime {
    NaiveTime::parse_from_str(day["astronomy"][0][key].as_str().unwrap(), "%I:%M %p")
        .unwrap()
}

pub fn format_ampm_time(time: NaiveTime, ampm: bool) -> String {
    if ampm {
        time.format("%I:%M %p").to_string()
    } else {
        time.format("%H:%M").to_string()
    }
}

pub fn format_indicator(
    weather_conditions: &Value,
    area: &Value,
    expression: String,
    weather_icon: &&str,
) -> String {
    if !weather_conditions.is_object() {
        return String::new();
    }

    let (weather_map, area_map) = match (weather_conditions.as_object(), area.as_object()) {
        (Some(w), Some(a)) => (w, a),
        _ => return String::new(),
    };
    let mut combined_map = weather_map.clone();
    combined_map.extend(area_map.clone());

    let mut formatted_indicator = expression.to_string();
    combined_map
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
