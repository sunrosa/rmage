#![cfg(test)]
mod model {
    use crate::model::*;

    fn moldgraf_monstrosity() -> Card {
        Card {
            name: "Moldgraf Monstrosity".to_string(),
            supertypes: vec![CardType::Creature],
            subtypes: vec!["Insect".to_string()],
            is_legendary: false,
            abilities: vec!["Trample".to_string(), "When Moldgraf Monstrosity dies, exile it, then return two creature cards at random from your graveyard to the battlefield.".to_string()],
            flavor_text: "The border between life and death is as thin as a layer of topsoil.".to_string(),
            power: 8,
            toughness: 8,
            color: vec![ManaColor::Green],
            color_identity: vec![ManaColor::Green],
            mana_cost: ManaCost {colorless: 4, colored: vec![(ManaColor::Green, 3)]},
            illustrator: "Tomasz Jedruszek".to_string(),
            set: Set {code: "C18".to_string(), name: "Commander 2018".to_string(), card_count: 307, release_date: chrono::NaiveDate::from_ymd(2018, 08, 09)},
            set_number: 156,
            rarity: Rarity::Rare,
            price: Some(Price {usd: 0.47, fetch_date: chrono::NaiveDate::from_ymd(2022, 09, 11)}),
            is_foil: false,
            legalities: Legalities {
                standard: Legality::NotLegal,
                pioneer: Legality::NotLegal,
                modern: Legality::Legal,
                legacy: Legality::Legal,
                vintage: Legality::Legal,
                commander: Legality::Legal,
                alchemy: Legality::NotLegal,
                explorer: Legality::NotLegal,
                brawl: Legality::NotLegal,
                historic: Legality::NotLegal,
                pauper: Legality::NotLegal,
                penny: Legality::Banned
            }
        }
    }

    #[test]
    pub fn card_cmc() {
        assert_eq!(moldgraf_monstrosity().cmc(), 7);
    }

    #[test]
    pub fn card_is_in_booster() {
        let mut test_card = moldgraf_monstrosity();
        test_card.set_number = 307;
        assert_eq!(test_card.is_in_booster(), true);
    }

    #[test]
    pub fn card_is_in_booster_1() {
        let mut test_card = moldgraf_monstrosity();
        test_card.set_number = 308;
        assert_eq!(test_card.is_in_booster(), false);
    }

    #[test]
    pub fn card_is_in_booster_2() {
        assert_eq!(moldgraf_monstrosity().is_in_booster(), true);
    }
}
