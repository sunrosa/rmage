pub mod model;

use model::*;

fn main() {
    let moldgraf_monstrosity = Card {
        name: "hi".to_string(),
        supertypes: vec![CardType::Creature],
        subtypes: vec!["Insect".to_string()],
        is_legendary: false,
        rules: vec!["Trample".to_string(), "When Moldgraf Monstrosity dies, exile it, then return two creature cards at random from your graveyard to the battlefield.".to_string()],
        flavor_text: "The border between life and death is as thin as a layer of topsoil.".to_string(),
        power: 8,
        toughness: 8,
        colors: vec![ManaColor::Green],
        mana_cost: ManaCost {colorless: 4, colored: vec![(ManaColor::Green, 3)]},
        illustrator: "Tomasz Jedruszek".to_string(),
        set: Set {code: "C18".to_string(), name: "Commander 2018".to_string(), card_count: 307, release_date: chrono::NaiveDate::from_ymd(2018, 08, 09)},
        set_number: 156,
        rarity: Rarity::Rare,
        price: Some(Price {usd: 0.47, fetch_date: chrono::NaiveDate::from_ymd(2022, 09, 11)}),
        is_foil: false
    };

    println!("{:?}", moldgraf_monstrosity);
}
