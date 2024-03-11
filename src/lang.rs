use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum Lang {
    EN,
    DE,
    PL,
    RU,
    TR,
}

impl Lang {
    pub fn wttr_in_subdomain(&self) -> String {
        match &self {
            Self::EN => "wttr.in".to_string(),
            Self::DE => "de.wttr.in".to_string(),
            Self::PL => "pl.wttr.in".to_string(),
            Self::RU => "ru.wttr.in".to_string(),
            Self::TR => "tr.wttr.in".to_string(),
        }
    }
    pub fn feels_like(&self) -> String {
        match &self {
            Self::EN => "Feels Like".to_string(),
            Self::DE => "Gefühlt wie".to_string(),
            Self::PL => "Temperatura odczuwalna".to_string(),
            Self::RU => "Ощущается как".to_string(),
            Self::TR => "Hissedilen".to_string(),
        }
    }
    pub fn humidity(&self) -> String {
        match &self {
            Self::EN => "humidity".to_string(),
            Self::DE => "Luftfeuchtigkeit".to_string(),
            Self::PL => "Wilgotność".to_string(),
            Self::RU => "Влажность".to_string(),
            Self::TR => "Nem".to_string(),
        }
    }
    pub fn location(&self) -> String {
        match &self {
            Self::EN => "Location".to_string(),
            Self::DE => "Standort".to_string(),
            Self::PL => "Lokalizacja".to_string(),
            Self::RU => "Местоположение".to_string(),
            Self::TR => "Konum".to_string(),
        }
    }
    pub fn today(&self) -> String {
        match &self {
            Self::EN => "Today".to_string(),
            Self::DE => "Heute".to_string(),
            Self::PL => "Dzisiaj".to_string(),
            Self::RU => "Сегодня".to_string(),
            Self::TR => "Bugün".to_string(),
        }
    }
    pub fn tomorrow(&self) -> String {
        match &self {
            Self::EN => "Tomorrow".to_string(),
            Self::DE => "Morgen".to_string(),
            Self::PL => "Jutro".to_string(),
            Self::RU => "Завтра".to_string(),
            Self::TR => "Yarın".to_string(),
        }
    }
    pub fn fog(&self) -> String {
        match &self {
            Self::EN => "Fog".to_string(),
            Self::DE => "Nebel".to_string(),
            Self::PL => "Mgła".to_string(),
            Self::RU => "Туман".to_string(),
            Self::TR => "Sis".to_string(),
        }
    }
    pub fn frost(&self) -> String {
        match &self {
            Self::EN => "Frost".to_string(),
            Self::DE => "Frost".to_string(),
            Self::PL => "Mróz".to_string(),
            Self::RU => "Мороз".to_string(),
            Self::TR => "Don".to_string(),
        }
    }
    pub fn overcast(&self) -> String {
        match &self {
            Self::EN => "Overcast".to_string(),
            Self::DE => "Bewölkung".to_string(),
            Self::PL => "Zachmurzenie".to_string(),
            Self::RU => "Пасмурно".to_string(),
            Self::TR => "Bulutlu".to_string(),
        }
    }
    pub fn rain(&self) -> String {
        match &self {
            Self::EN => "Rain".to_string(),
            Self::DE => "Regen".to_string(),
            Self::PL => "Deszcz".to_string(),
            Self::RU => "Дождь".to_string(),
            Self::TR => "Yağmur".to_string(),
        }
    }
    pub fn snow(&self) -> String {
        match &self {
            Self::EN => "Snow".to_string(),
            Self::DE => "Schnee".to_string(),
            Self::PL => "Śnieg".to_string(),
            Self::RU => "Снег".to_string(),
            Self::TR => "Kar".to_string(),
        }
    }
    pub fn sunshine(&self) -> String {
        match &self {
            Self::EN => "Sunshine".to_string(),
            Self::DE => "Sonnenschein".to_string(),
            Self::PL => "Nasłonecznienie".to_string(),
            Self::RU => "Солнечно".to_string(),
            Self::TR => "Güneş ışığı".to_string(),
        }
    }
    pub fn thunder(&self) -> String {
        match &self {
            Self::EN => "Thunder".to_string(),
            Self::DE => "Donner".to_string(),
            Self::PL => "Burza".to_string(),
            Self::RU => "Гроза".to_string(),
            Self::TR => "Gök gürültüsü".to_string(),
        }
    }
    pub fn wind(&self) -> String {
        match &self {
            Self::EN => "Wind".to_string(),
            Self::DE => "Wind".to_string(),
            Self::PL => "Wiatr".to_string(),
            Self::RU => "Ветер".to_string(),
            Self::TR => "Rüzgar".to_string(),
        }
    }
    pub fn weather_desc(&self) -> String {
        match &self {
            Lang::EN => "weatherDesc".to_string(),
            Lang::DE => "lang_de".to_string(),
            Lang::PL => "lang_pl".to_string(),
            Lang::RU => "lang_ru".to_string(),
            Lang::TR => "lang_tr".to_string(),
        }
    }
}
