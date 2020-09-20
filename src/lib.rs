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
                HanafudaCard::new(Month::January, Plant::Pine, String::from("Crane and Sun"), CardType::Bright, 20, String::from("img/01.png")),
                HanafudaCard::new(Month::January, Plant::Pine, String::from("Poetry Tanzaku"), CardType::PoetryTanzaku, 5, String::from("img/02.png")),
                HanafudaCard::new(Month::January, Plant::Pine, String::from("Kasu"), CardType::Kasu, 1, String::from("img/03.png")),
                HanafudaCard::new(Month::January, Plant::Pine, String::from("Kasu"), CardType::Kasu, 1, String::from("img/04.png")),
                HanafudaCard::new(Month::February, Plant::PlumBlossom, String::from("Bush Warbler"), CardType::Animal, 10, String::from("img/05.png")),
                HanafudaCard::new(Month::February, Plant::PlumBlossom, String::from("Poetry Tanzaku"), CardType::PoetryTanzaku, 5, String::from("img/06.png")),
                HanafudaCard::new(Month::February, Plant::PlumBlossom, String::from("Kasu"), CardType::Kasu, 1, String::from("img/07.png")),
                HanafudaCard::new(Month::February, Plant::PlumBlossom, String::from("Kasu"), CardType::Kasu, 1, String::from("img/08.png")),
                HanafudaCard::new(Month::March, Plant::CherryBlossom, String::from("Curtain"), CardType::Bright, 20, String::from("img/09.png")),
                HanafudaCard::new(Month::March, Plant::CherryBlossom, String::from("Poetry Tanzaku"), CardType::PoetryTanzaku, 5, String::from("img/10.png")),
                HanafudaCard::new(Month::March, Plant::CherryBlossom, String::from("Kasu"), CardType::Kasu, 1, String::from("img/11.png")),
                HanafudaCard::new(Month::March, Plant::CherryBlossom, String::from("Kasu"), CardType::Kasu, 1, String::from("img/12.png")),
                HanafudaCard::new(Month::April, Plant::Wisteria, String::from("Cuckoo"), CardType::Animal, 10, String::from("img/13.png")),
                HanafudaCard::new(Month::April, Plant::Wisteria, String::from("Tanzaku"), CardType::Tanzaku, 5, String::from("img/14.png")),
                HanafudaCard::new(Month::April, Plant::Wisteria, String::from("Kasu"), CardType::Kasu, 1, String::from("img/15.png")),
                HanafudaCard::new(Month::April, Plant::Wisteria, String::from("Kasu"), CardType::Kasu, 1, String::from("img/16.png")),
                HanafudaCard::new(Month::May, Plant::Iris, String::from("Eight-Plank Bridge"), CardType::Animal, 10, String::from("img/17.png")),
                HanafudaCard::new(Month::May, Plant::Iris, String::from("Tanzaku"), CardType::Tanzaku, 5, String::from("img/18.png")),
                HanafudaCard::new(Month::May, Plant::Iris, String::from("Kasu"), CardType::Kasu, 1, String::from("img/19.png")),
                HanafudaCard::new(Month::May, Plant::Iris, String::from("Kasu"), CardType::Kasu, 1, String::from("img/20.png")),
                HanafudaCard::new(Month::June, Plant::Peony, String::from("Butterflies"), CardType::Animal, 10, String::from("img/21.png")),
                HanafudaCard::new(Month::June, Plant::Peony, String::from("Blue Tanzaku"), CardType::BlueTanzaku, 5, String::from("img/22.png")),
                HanafudaCard::new(Month::June, Plant::Peony, String::from("Kasu"), CardType::Kasu, 1, String::from("img/23.png")),
                HanafudaCard::new(Month::June, Plant::Peony, String::from("Kasu"), CardType::Kasu, 1, String::from("img/24.png")),
                HanafudaCard::new(Month::July, Plant::BushClover, String::from("Boar"), CardType::Animal, 10, String::from("img/25.png")),
                HanafudaCard::new(Month::July, Plant::BushClover, String::from("Tanzaku"), CardType::Tanzaku, 5, String::from("img/26.png")),
                HanafudaCard::new(Month::July, Plant::BushClover, String::from("Kasu"), CardType::Kasu, 1, String::from("img/27.png")),
                HanafudaCard::new(Month::July, Plant::BushClover, String::from("Kasu"), CardType::Kasu, 1, String::from("img/28.png")),
                HanafudaCard::new(Month::August, Plant::SusukiGrass, String::from("Full Moon"), CardType::Bright, 20, String::from("img/29.png")),
                HanafudaCard::new(Month::August, Plant::SusukiGrass, String::from("Geese"), CardType::Animal, 10, String::from("img/30.png")),
                HanafudaCard::new(Month::August, Plant::SusukiGrass, String::from("Kasu"), CardType::Kasu, 1, String::from("img/31.png")),
                HanafudaCard::new(Month::August, Plant::SusukiGrass, String::from("Kasu"), CardType::Kasu, 1, String::from("img/32.png")),
                HanafudaCard::new(Month::September, Plant::Chrysanthemum, String::from("Sake Cup"), CardType::Animal, 10, String::from("img/33.png")),
                HanafudaCard::new(Month::September, Plant::Chrysanthemum, String::from("Blue Tanzaku"), CardType::BlueTanzaku, 5, String::from("img/34.png")),
                HanafudaCard::new(Month::September, Plant::Chrysanthemum, String::from("Kasu"), CardType::Kasu, 1, String::from("img/35.png")),
                HanafudaCard::new(Month::September, Plant::Chrysanthemum, String::from("Kasu"), CardType::Kasu, 1, String::from("img/36.png")),
                HanafudaCard::new(Month::October, Plant::Maple, String::from("Deer"), CardType::Animal, 10, String::from("img/37.png")),
                HanafudaCard::new(Month::October, Plant::Maple, String::from("Blue Tanzaku"), CardType::Tanzaku, 5, String::from("img/38.png")),
                HanafudaCard::new(Month::October, Plant::Maple, String::from("Kasu"), CardType::Kasu, 1, String::from("img/39.png")),
                HanafudaCard::new(Month::October, Plant::Maple, String::from("Kasu"), CardType::Kasu, 1, String::from("img/40.png")),
                HanafudaCard::new(Month::November, Plant::Willow, String::from("Ono no Michikaze"), CardType::Bright, 20, String::from("img/41.png")),
                HanafudaCard::new(Month::November, Plant::Willow, String::from("Swallow"), CardType::Animal, 10, String::from("img/42.png")),
                HanafudaCard::new(Month::November, Plant::Willow, String::from("Tanzaku"), CardType::Tanzaku, 5, String::from("img/43.png")),
                HanafudaCard::new(Month::November, Plant::Willow, String::from("Kasu"), CardType::Lightning, 1, String::from("img/44.png")),
                HanafudaCard::new(Month::December, Plant::Paulownia, String::from("Phoenix"), CardType::Bright, 20, String::from("img/45.png")),
                HanafudaCard::new(Month::December, Plant::Paulownia, String::from("Kasu"), CardType::Kasu, 1, String::from("img/46.png")),
                HanafudaCard::new(Month::December, Plant::Paulownia, String::from("Kasu"), CardType::Kasu, 1, String::from("img/47.png")),
                HanafudaCard::new(Month::December, Plant::Paulownia, String::from("Kasu"), CardType::Kasu, 1, String::from("img/48.png"))
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
