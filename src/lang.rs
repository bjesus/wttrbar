use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum Lang {
    EN,
    DE,
    PL,
    RU,
    TR,
    FR,
    BE,
    ZH,
    ES,
    PT,
    IT,
    JA,
}

impl Lang {
    pub fn wttr_in_subdomain(&self) -> String {
        match &self {
            Self::EN => "wttr.in".to_string(),
            Self::DE => "de.wttr.in".to_string(),
            Self::PL => "pl.wttr.in".to_string(),
            Self::RU => "ru.wttr.in".to_string(),
            Self::TR => "tr.wttr.in".to_string(),
            Self::FR => "fr.wttr.in".to_string(),
            Self::BE => "be.wttr.in".to_string(),
            Self::ZH => "zh.wttr.in".to_string(),
            Self::ES => "es.wttr.in".to_string(),
            Self::PT => "pt.wttr.in".to_string(),
            Self::IT => "it.wttr.in".to_string(),
            Self::JA => "ja.wttr.in".to_string(),
        }
    }
    pub fn feels_like(&self) -> String {
        match &self {
            Self::EN => "Feels Like".to_string(),
            Self::DE => "Gefühlt wie".to_string(),
            Self::PL => "Temperatura odczuwalna".to_string(),
            Self::RU => "Ощущается как".to_string(),
            Self::TR => "Hissedilen".to_string(),
            Self::FR => "Ressenti".to_string(),
            Self::BE => "Адчуваецца як".to_string(),
            Self::ZH => "体感温度".to_string(),
            Self::ES => "Sensación térmica".to_string(),
            Self::PT => "Sensação térmica".to_string(),
            Self::IT => "Sensazione Termica".to_string(),
            Self::JA => "体感温度".to_string(),
        }
    }
    pub fn humidity(&self) -> String {
        match &self {
            Self::EN => "Humidity".to_string(),
            Self::DE => "Luftfeuchtigkeit".to_string(),
            Self::PL => "Wilgotność".to_string(),
            Self::RU => "Влажность".to_string(),
            Self::TR => "Nem".to_string(),
            Self::FR => "Humidité".to_string(),
            Self::BE => "Вільготнасць".to_string(),
            Self::ZH => "湿度".to_string(),
            Self::ES => "Humedad".to_string(),
            Self::PT => "Umidade".to_string(),
            Self::IT => "Umidità".to_string(),
            Self::JA => "湿度".to_string(),
        }
    }
    pub fn location(&self) -> String {
        match &self {
            Self::EN => "Location".to_string(),
            Self::DE => "Standort".to_string(),
            Self::PL => "Lokalizacja".to_string(),
            Self::RU => "Местоположение".to_string(),
            Self::TR => "Konum".to_string(),
            Self::FR => "Lieu".to_string(),
            Self::BE => "Месцазнаходжанне".to_string(),
            Self::ZH => "地区".to_string(),
            Self::ES => "Ubicación".to_string(),
            Self::PT => "Localização".to_string(),
            Self::IT => "Posizione".to_string(),
            Self::JA => "地点".to_string(),
        }
    }
    pub fn today(&self) -> String {
        match &self {
            Self::EN => "Today".to_string(),
            Self::DE => "Heute".to_string(),
            Self::PL => "Dzisiaj".to_string(),
            Self::RU => "Сегодня".to_string(),
            Self::TR => "Bugün".to_string(),
            Self::FR => "Aujourd'hui".to_string(),
            Self::BE => "Сёння".to_string(),
            Self::ZH => "今日天气".to_string(),
            Self::ES => "Hoy".to_string(),
            Self::PT => "Hoje".to_string(),
            Self::IT => "Oggi".to_string(),
            Self::JA => "今日".to_string(),
        }
    }
    pub fn tomorrow(&self) -> String {
        match &self {
            Self::EN => "Tomorrow".to_string(),
            Self::DE => "Morgen".to_string(),
            Self::PL => "Jutro".to_string(),
            Self::RU => "Завтра".to_string(),
            Self::TR => "Yarın".to_string(),
            Self::FR => "Demain".to_string(),
            Self::BE => "Заўтра".to_string(),
            Self::ZH => "明日天气".to_string(),
            Self::ES => "Mañana".to_string(),
            Self::PT => "Amanhã".to_string(),
            Self::IT => "Domani".to_string(),
            Self::JA => "明日".to_string(),
        }
    }
    pub fn fog(&self) -> String {
        match &self {
            Self::EN => "Fog".to_string(),
            Self::DE => "Nebel".to_string(),
            Self::PL => "Mgła".to_string(),
            Self::RU => "Туман".to_string(),
            Self::TR => "Sis".to_string(),
            Self::FR => "Brouillard".to_string(),
            Self::BE => "Туман".to_string(),
            Self::ZH => "雾".to_string(),
            Self::ES => "Niebla".to_string(),
            Self::PT => "Nevoeiro".to_string(),
            Self::IT => "Nebbia".to_string(),
            Self::JA => "霧".to_string(),
        }
    }
    pub fn frost(&self) -> String {
        match &self {
            Self::EN => "Frost".to_string(),
            Self::DE => "Frost".to_string(),
            Self::PL => "Mróz".to_string(),
            Self::RU => "Мороз".to_string(),
            Self::TR => "Don".to_string(),
            Self::FR => "Gel".to_string(),
            Self::BE => "Мароз".to_string(),
            Self::ZH => "霜".to_string(),
            Self::ES => "Escarcha".to_string(),
            Self::PT => "Geada".to_string(),
            Self::IT => "Gelo".to_string(),
            Self::JA => "霜".to_string(),
        }
    }
    pub fn overcast(&self) -> String {
        match &self {
            Self::EN => "Overcast".to_string(),
            Self::DE => "Bewölkung".to_string(),
            Self::PL => "Zachmurzenie".to_string(),
            Self::RU => "Пасмурно".to_string(),
            Self::TR => "Bulutlu".to_string(),
            Self::FR => "Couvert".to_string(),
            Self::BE => "Хмурна".to_string(),
            Self::ZH => "多云".to_string(),
            Self::ES => "Nublado".to_string(),
            Self::PT => "Nublado".to_string(),
            Self::IT => "Nuvoloso".to_string(),
            Self::JA => "曇り".to_string(),
        }
    }
    pub fn rain(&self) -> String {
        match &self {
            Self::EN => "Rain".to_string(),
            Self::DE => "Regen".to_string(),
            Self::PL => "Deszcz".to_string(),
            Self::RU => "Дождь".to_string(),
            Self::TR => "Yağmur".to_string(),
            Self::FR => "Pluie".to_string(),
            Self::BE => "Дождж".to_string(),
            Self::ZH => "雨".to_string(),
            Self::ES => "Lluvia".to_string(),
            Self::PT => "Chuva".to_string(),
            Self::IT => "Pioggia".to_string(),
            Self::JA => "雨".to_string(),
        }
    }
    pub fn snow(&self) -> String {
        match &self {
            Self::EN => "Snow".to_string(),
            Self::DE => "Schnee".to_string(),
            Self::PL => "Śnieg".to_string(),
            Self::RU => "Снег".to_string(),
            Self::TR => "Kar".to_string(),
            Self::FR => "Neige".to_string(),
            Self::BE => "Снег".to_string(),
            Self::ZH => "雪".to_string(),
            Self::ES => "Nieve".to_string(),
            Self::PT => "Neve".to_string(),
            Self::IT => "Neve".to_string(),
            Self::JA => "雪".to_string(),
        }
    }
    pub fn sunshine(&self) -> String {
        match &self {
            Self::EN => "Sunshine".to_string(),
            Self::DE => "Sonnenschein".to_string(),
            Self::PL => "Nasłonecznienie".to_string(),
            Self::RU => "Солнечно".to_string(),
            Self::TR => "Güneş ışığı".to_string(),
            Self::FR => "Ensoleillé".to_string(),
            Self::BE => "Сонечна".to_string(),
            Self::ZH => "晴".to_string(),
            Self::ES => "Soleado".to_string(),
            Self::PT => "Sol".to_string(),
            Self::IT => "Sole".to_string(),
            Self::JA => "晴れ".to_string(),
        }
    }
    pub fn thunder(&self) -> String {
        match &self {
            Self::EN => "Thunder".to_string(),
            Self::DE => "Donner".to_string(),
            Self::PL => "Burza".to_string(),
            Self::RU => "Гроза".to_string(),
            Self::TR => "Gök gürültüsü".to_string(),
            Self::FR => "Orages".to_string(),
            Self::BE => "Навальніца".to_string(),
            Self::ZH => "雷暴".to_string(),
            Self::ES => "Tormenta".to_string(),
            Self::PT => "Trovão".to_string(),
            Self::IT => "Tuono".to_string(),
            Self::JA => "雷".to_string(),
        }
    }
    pub fn wind(&self) -> String {
        match &self {
            Self::EN => "Wind".to_string(),
            Self::DE => "Wind".to_string(),
            Self::PL => "Wiatr".to_string(),
            Self::RU => "Ветер".to_string(),
            Self::TR => "Rüzgar".to_string(),
            Self::FR => "Vent".to_string(),
            Self::BE => "Вецер".to_string(),
            Self::ZH => "风速".to_string(),
            Self::ES => "Viento".to_string(),
            Self::PT => "Vento".to_string(),
            Self::IT => "Vento".to_string(),
            Self::JA => "風速".to_string(),
        }
    }
    pub fn weather_desc(&self) -> String {
        match &self {
            Lang::EN => "weatherDesc".to_string(),
            Lang::DE => "lang_de".to_string(),
            Lang::PL => "lang_pl".to_string(),
            Lang::RU => "lang_ru".to_string(),
            Lang::TR => "lang_tr".to_string(),
            Lang::FR => "lang_fr".to_string(),
            Lang::BE => "lang_be".to_string(),
            Lang::ZH => "lang_zh".to_string(),
            Lang::ES => "lang_es".to_string(),
            Lang::PT => "lang_pt".to_string(),
            Lang::IT => "lang_it".to_string(),
            Lang::JA => "lang_ja".to_string(),
        }
    }
}
