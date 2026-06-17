pub const WEATHER_CODES: &[(i32, &str)] = &[
    (113, "☀️"), // Sunny
    (116, "🌤️"), // Partly cloudly
    (119, "☁️"), // Cloudy
    (122, "🌥️"), // Very cloudy
    (125, "🌫️"), // Haze
    (128, "🌫️"), // Dust haze
    (131, "🌫️"), // Blowing dust
    (134, "🌫️"), // Dust storm
    (137, "🌫️"), // Sandstorm
    (140, "🌫️"), // Severe sandstorm
    (143, "🌫️"), // Fog
    (146, "🌫️"), // Smoke
    (149, "🌫️"), // Smoky haze
    (152, "🌫️"), // Smog
    (155, "🌫️"), // Severe smog
    (158, "🌫️"), // Saharan dust
    (161, "🌫️"), // Dust
    (176, "🌦️"), // Light showers
    (179, "🌧️"), // Light sleet showers
    (182, "🌧️"), // Light sleet
    (185, "🌧️"), // Light sleet
    (200, "🌩️"), // Thundery showers
    (227, "❄️"), // Light snow
    (230, "❄️"), // Heavy snow
    (248, "🌫️"), // Fog
    (260, "🌫️"), // Fog
    (263, "🌧️"), // Light showers
    (266, "🌧️"), // Light rain
    (281, "🌦️"), // Light sleet
    (284, "🌦️"), // Light sleet
    (293, "🌧️"), // Light rain
    (296, "🌧️"), // Light rain
    (299, "🌧️"), // Heavy showers
    (302, "🌧️"), // Heavy rain
    (305, "🌧️"), // Heavy showers
    (308, "🌧️"), // Heavy rain
    (311, "🌧️"), // Light sleet
    (314, "🌧️"), // Light sleet
    (317, "🌧️"), // Light sleet
    (320, "🌨️"), // Light snow
    (323, "🌨️"), // Light snow showers
    (326, "🌨️"), // Light snow showers
    (329, "🌨️"), // Heavy Snow
    (332, "🌨️"), // Heavy Snow
    (335, "🌨️"), // Heavy snow showers
    (338, "🌨️"), // Heavy snow
    (350, "🌨️"), // Light sleet
    (353, "🌧️"), // Light showers
    (356, "🌧️"), // Heavy showers
    (359, "🌧️"), // Heavy rain
    (362, "🌨️"), // Light sleet showers
    (365, "🌨️"), // Light sleet showers
    (368, "🌨️"), // Light snow showers
    (371, "🌨️"), // Heavy snow showers
    (374, "🌨️"), // Light sleet showers
    (377, "🌨️"), // Light sleet
    (386, "🌩️"), // Thundery showers
    (389, "🌨️"), // Thundery heavy rain
    (392, "🌨️"), // Thundery snow showers
    (395, "🌨️"), // Heavy snow showers
    (398, "🌨️"), // This is all the ones defined in the wttr.in source code, not sure what these are, but apparently some sort of rain.
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

pub const WEATHER_CODES_NERD: &[(i32, &str)] = &[
    (113, "󰖙"), // Sunny
    (116, "󰖕"), // Partly cloudly
    (119, "󰼰"), // Cloudy
    (122, "󰖐"), // Very cloudy
    (125, "󰖑"), // Haze
    (128, "󰖑"), // Dust haze
    (131, "󰖑"), // Blowing dust
    (134, "󰖑"), // Dust storm
    (137, "󰖑"), // Sandstorm
    (140, "󰖑"), // Severe sandstorm
    (143, "󰖑"), // Fog
    (146, "󰖑"), // Smoke
    (149, "󰖑"), // Smoky haze
    (152, "󰖑"), // Smog
    (155, "󰖑"), // Severe smog
    (158, "󰖑"), // Saharan dust
    (161, "󰖑"), // Dust
    (176, "󰖗"), // Light showers
    (179, "󰙿"), // Light sleet showers
    (182, "󰙿"), // Light sleet
    (185, "󰙿"), // Light sleet
    (200, "󰙾"), // Thundery showers
    (227, "󰖘"), // Light snow
    (230, "󰼶"), // Heavy snow
    (248, "󰖑"), // Fog
    (260, "󰖑"), // Fog
    (263, "󰖗"), // Light showers
    (266, "󰖗"), // Light rain
    (281, "󰙿"), // Light sleet
    (284, "󰙿"), // Light sleet
    (293, "󰖗"), // Light rain
    (296, "󰖗"), // Light rain
    (299, "󰖖"), // Heavy showers
    (302, "󰖖"), // Heavy rain
    (305, "󰖖"), // Heavy showers
    (308, "󰖖"), // Heavy rain
    (311, "󰙿"), // Light sleet
    (314, "󰙿"), // Light sleet
    (317, "󰙿"), // Light sleet
    (320, "󰖘"), // Light snow
    (323, "󰖘"), // Light snow showers
    (326, "󰖘"), // Light snow showers
    (329, "󰼶"), // Heavy Snow
    (332, "󰼶"), // Heavy Snow
    (335, "󰼶"), // Heavy snow showers
    (338, "󰼶"), // Heavy snow
    (350, "󰙿"), // Light sleet
    (353, "󰖗"), // Light showers
    (356, "󰖖"), // Heavy showers
    (359, "󰖖"), // Heavy rain
    (362, "󰙿"), // Light sleet showers
    (365, "󰙿"), // Light sleet showers
    (368, "󰖘"), // Light snow showers
    (371, "󰼶"), // Heavy snow showers
    (374, "󰙿"), // Light sleet showers
    (377, "󰙿"), // Light sleet
    (386, "󰙾"), // Thundery showers
    (389, "󰙾"), // Thundery heavy rain
    (392, "󰙾"), // Thundery snow showers
    (395, "󰼶"), // Heavy snow showers
    (398, "󰖗"), // This is all the ones defined in the wttr.in source code, not sure what these are, but apparently some sort of rain.
    (401, "󰖗"),
    (404, "󰖗"),
    (407, "󰖗"),
    (410, "󰖗"),
    (413, "󰖗"),
    (416, "󰖗"),
    (419, "󰖗"),
    (422, "󰖗"),
    (425, "󰖗"),
    (428, "󰖗"),
    (431, "󰖗"),
];

pub const MOON_PHASES: &[(&str, &str)] = &[
    ("New Moon", "🌑"),
    ("Waxing Crescent", "🌒"),
    ("First Quarter", "🌓"),
    ("Waxing Gibbous", "🌔"),
    ("Full Moon", "🌕"),
    ("Waning Gibbous", "🌖"),
    ("Last Quarter", "🌗"),
    ("Waning Crescent", "🌘"),
];

pub const MOON_PHASES_NERD: &[(&str, &str)] = &[
    ("New Moon", "󰽤"),
    ("Waxing Crescent", "󰽧"),
    ("First Quarter", "󰽡"),
    ("Waxing Gibbous", "󰽨"),
    ("Full Moon", "󰽢"),
    ("Waning Gibbous", "󰽦"),
    ("Last Quarter", "󰽣"),
    ("Waning Crescent", "󰽥"),
];

pub const ICON_PLACEHOLDER: &str = "{ICON}";

#[cfg(test)]
mod tests {
    #[test]
    fn test_weather_code() {
        let weather_code =
            reqwest::blocking::get("https://www.worldweatheronline.com/feed/wwoConditionCodes.txt")
                .expect("Get weather code from www.worldweatheronline.com success")
                .text()
                .expect("Decode http response from www.worldweatheronline.com success");
        let mut wc_lines = weather_code.lines();

        // skip header line
        wc_lines.next();

        for wc in wc_lines {
            let code = wc
                .split_whitespace()
                .next()
                .expect("First field is weather code")
                .parse::<i32>()
                .expect("Parse weather code to i32");
            assert!(
                crate::constants::WEATHER_CODES
                    .iter()
                    .find(|(c, _)| *c == code)
                    .is_some(),
                "WEATHER_CODES missing a weather code {code}."
            );
            assert!(
                crate::constants::WEATHER_CODES_NERD
                    .iter()
                    .find(|(c, _)| *c == code)
                    .is_some(),
                "WEATHER_CODES_NERD missing a weather code {code}."
            );
        }
    }
}
