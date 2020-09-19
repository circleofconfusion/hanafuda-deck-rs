use std::cmp::Ordering;
use std::env;
use rand::Rng;

pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December
}

pub enum Plant {
    Pine,
    PlumBlossom,
    CherryBlossom,
    Wisteria,
    Iris,
    Peony,
    BushClover,
    SusukiGrass,
    Chrysanthemum,
    Maple,
    Willow,
    Paulownia
}

pub enum CardType {
    Bright,
    Tanzaku,
    BlueTanzaku,
    PoetryTanzaku,
    Animal,
    Kasu,
    Lightning
}

pub struct HanafudaCard {
    pub month: Month,
    pub plant: Plant,
    pub face: String,
    pub card_type: CardType,
    pub value: u8,
    pub image_path: String
}

impl HanafudaCard {
    pub fn new(month: Month, plant: Plant, face: String, card_type: CardType, value: u8, image_path: String) -> Self {
        HanafudaCard {
            month,
            plant,
            face,
            card_type,
            value,
            image_path
        }
    }
}

impl PartialEq for HanafudaCard {
    fn eq(&self, other: &HanafudaCard) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for HanafudaCard {
    fn partial_cmp(&self, other: &HanafudaCard) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

pub struct HanafudaDeck {
    pub cards: Vec<HanafudaCard>
}

impl HanafudaDeck {
    pub fn new() -> Self {
        HanafudaDeck {
            cards: vec![
                HanafudaCard::new(Month::January, Plant::Pine, String::from("Crane and Sun"), CardType::Bright, 20, String::from(format!("{}/img/01.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::January, Plant::Pine, String::from("Poetry Tanzaku"), CardType::PoetryTanzaku, 5, String::from(format!("{}/img/02.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::January, Plant::Pine, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/03.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::January, Plant::Pine, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/04.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::February, Plant::PlumBlossom, String::from("Bush Warbler"), CardType::Animal, 10, String::from(format!("{}/img/05.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::February, Plant::PlumBlossom, String::from("Poetry Tanzaku"), CardType::PoetryTanzaku, 5, String::from(format!("{}/img/06.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::February, Plant::PlumBlossom, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/07.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::February, Plant::PlumBlossom, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/08.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::March, Plant::CherryBlossom, String::from("Curtain"), CardType::Bright, 20, String::from(format!("{}/img/09.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::March, Plant::CherryBlossom, String::from("Poetry Tanzaku"), CardType::PoetryTanzaku, 5, String::from(format!("{}/img/10.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::March, Plant::CherryBlossom, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/11.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::March, Plant::CherryBlossom, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/12.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::April, Plant::Wisteria, String::from("Cuckoo"), CardType::Animal, 10, String::from(format!("{}/img/13.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::April, Plant::Wisteria, String::from("Tanzaku"), CardType::Tanzaku, 5, String::from(format!("{}/img/14.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::April, Plant::Wisteria, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/15.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::April, Plant::Wisteria, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/16.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::May, Plant::Iris, String::from("Eight-Plank Bridge"), CardType::Animal, 10, String::from(format!("{}/img/17.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::May, Plant::Iris, String::from("Tanzaku"), CardType::Tanzaku, 5, String::from(format!("{}/img/18.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::May, Plant::Iris, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/19.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::May, Plant::Iris, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/20.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::June, Plant::Peony, String::from("Butterflies"), CardType::Animal, 10, String::from(format!("{}/img/21.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::June, Plant::Peony, String::from("Blue Tanzaku"), CardType::BlueTanzaku, 5, String::from(format!("{}/img/22.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::June, Plant::Peony, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/23.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::June, Plant::Peony, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/24.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::July, Plant::BushClover, String::from("Boar"), CardType::Animal, 10, String::from(format!("{}/img/25.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::July, Plant::BushClover, String::from("Tanzaku"), CardType::Tanzaku, 5, String::from(format!("{}/img/26.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::July, Plant::BushClover, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/27.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::July, Plant::BushClover, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/28.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::August, Plant::SusukiGrass, String::from("Full Moon"), CardType::Bright, 20, String::from(format!("{}/img/29.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::August, Plant::SusukiGrass, String::from("Geese"), CardType::Animal, 10, String::from(format!("{}/img/30.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::August, Plant::SusukiGrass, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/31.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::August, Plant::SusukiGrass, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/32.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::September, Plant::Chrysanthemum, String::from("Sake Cup"), CardType::Animal, 10, String::from(format!("{}/img/33.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::September, Plant::Chrysanthemum, String::from("Blue Tanzaku"), CardType::BlueTanzaku, 5, String::from(format!("{}/img/34.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::September, Plant::Chrysanthemum, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/35.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::September, Plant::Chrysanthemum, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/36.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::October, Plant::Maple, String::from("Deer"), CardType::Animal, 10, String::from(format!("{}/img/37.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::October, Plant::Maple, String::from("Blue Tanzaku"), CardType::Tanzaku, 5, String::from(format!("{}/img/38.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::October, Plant::Maple, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/39.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::October, Plant::Maple, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/40.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::November, Plant::Willow, String::from("Ono no Michikaze"), CardType::Bright, 20, String::from(format!("{}/img/41.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::November, Plant::Willow, String::from("Swallow"), CardType::Animal, 10, String::from(format!("{}/img/42.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::November, Plant::Willow, String::from("Tanzaku"), CardType::Tanzaku, 5, String::from(format!("{}/img/43.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::November, Plant::Willow, String::from("Kasu"), CardType::Lightning, 1, String::from(format!("{}/img/44.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::December, Plant::Paulownia, String::from("Phoenix"), CardType::Bright, 20, String::from(format!("{}/img/45.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::December, Plant::Paulownia, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/46.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::December, Plant::Paulownia, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/47.png", env::var("OUT_DIR").unwrap()))),
                HanafudaCard::new(Month::December, Plant::Paulownia, String::from("Kasu"), CardType::Kasu, 1, String::from(format!("{}/img/48.png", env::var("OUT_DIR").unwrap())))
            ]
        }
    }
        
    pub fn shuffle(&mut self) {
        for i in 0..self.cards.len() {
            let j = rand::thread_rng().gen_range(i, 48);
            self.cards.swap(i, j);
        }
    }
}
