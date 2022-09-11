#![cfg(test)]
mod model {
    use crate::model::*;
    #[test]
    pub fn card_cmc() {
        let moldgraf_monstrosity = Card {
        name: "Moldgraf Monstrosity".to_string(),
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

        assert_eq!(moldgraf_monstrosity.cmc(), 7);
    }

    #[test]
    pub fn card_is_in_booster() {
        let test_card = Card {
        name: "TESTCARD".to_string(),
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
        set_number: 307,
        rarity: Rarity::Rare,
        price: Some(Price {usd: 0.47, fetch_date: chrono::NaiveDate::from_ymd(2022, 09, 11)}),
        is_foil: false
    };

        assert_eq!(test_card.is_in_booster(), true);
    }
    #[test]
    pub fn card_is_in_booster_1() {
        let moldgraf_monstrosity = Card {
        name: "Moldgraf Monstrosity".to_string(),
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

        assert_eq!(moldgraf_monstrosity.is_in_booster(), true);
    }
}
