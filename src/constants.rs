pub const WEATHER_CODES_NOTO: &[(i32, &str)] = &[
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

pub const WEATHER_CODES_NERD: &[(i32, &str)] = &[
    (113, ""),
    (116, ""),
    (119, ""),
    (122, ""),
    (143, ""),
    (176, ""),
    (179, ""),
    (182, ""),
    (185, ""),
    (200, ""),
    (227, ""),
    (230, ""),
    (248, ""),
    (260, ""),
    (263, ""),
    (266, ""),
    (281, ""),
    (284, ""),
    (293, ""),
    (296, ""),
    (299, ""),
    (302, ""),
    (305, ""),
    (308, ""),
    (311, ""),
    (314, ""),
    (317, ""),
    (320, ""),
    (323, ""),
    (326, ""),
    (329, ""),
    (332, ""),
    (335, ""),
    (338, ""),
    (350, ""),
    (353, ""),
    (356, ""),
    (359, ""),
    (362, ""),
    (365, ""),
    (368, ""),
    (371, ""),
    (374, ""),
    (377, ""),
    (386, ""),
    (389, ""),
    (392, ""),
    (395, ""),
    (398, ""),
    (401, ""),
    (404, ""),
    (407, ""),
    (410, ""),
    (413, ""),
    (416, ""),
    (419, ""),
    (422, ""),
    (425, ""),
    (428, ""),
    (431, ""),
];

pub const SUNRISE_SUNSET_ICONS: &[(&str, (&str, &str))] =
    &[("noto", ("🌅 ", "🌇")), ("nerd", (" ", ""))];

pub const MIN_MAX_TEMP_ICONS: &[(&str, (&str, &str))] =
    &[("noto", ("⬇️ ", "⬆️")), ("nerd", ("", ""))];

pub const ICON_PLACEHOLDER: &str = "{ICON}";
