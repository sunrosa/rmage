use chrono::NaiveDate;

/// A Magic the Gathering card.
#[derive(Debug)]
pub struct Card {
    /// The name of the card, as seen above the artwork, to the left of the casting cost.
    pub name: String,
    /// The supertype(s) of the card (example: "Creature", "Enchantment Creature", "Planeswalker").
    pub supertypes: Vec<CardType>,
    /// The subtypes of the card (example: "God", "Human Soldier", "Elemental Giant", "Aura")
    pub subtypes: Vec<String>,
    /// Whether the card is legendary or not (seen as the leftmost supertype of the card).
    pub is_legendary: bool,
    /// The rules, activated abilities, triggered abilities, keywords, etc, of the card.
    pub rules: Vec<String>,
    /// The flavor text (in italics) of the card, seen below its rules text.
    pub flavor_text: String,
    /// The power of a creature card, seen as the left number before the slash in front of its toughness. The numbers are located in the bottom right of the card.
    pub power: i32,
    /// The toughness of a creature card, seen as the right number after the slash after its power. The numbers are located in the bottom right of the card.
    pub toughness: i32,
    /// The color identities of the card (seen typically in its border).
    pub colors: Vec<ManaColor>,
    /// The mana costs of the card (which makes it a spell). It's seen in the top right of the card.
    pub mana_cost: ManaCost,
    /// The artist of the artwork in the card, seen in the bottom left of the card in very small text, to the right of a brush symbol.
    pub illustrator: String,
    /// The set of the card (the specific origin of the printing of the card).
    pub set: Set,
    /// The index of the card in its set.
    pub set_number: i32,
    /// The rarity of the card, seen as the color of the set symbol on the right side of the type field of the card.
    pub rarity: Rarity,
    /// (Optional) price of the card.
    pub price: Option<Price>,
    /// Whether the card has a foil (appears iridescent) over it or not.
    pub is_foil: bool,
}

impl Card {
    /// The converted mana cost to cast the card (not including activated abilities and any rulings).
    pub fn cmc(&self) -> i32 {
        let mut cmc = self.mana_cost.colorless;
        for color in &self.mana_cost.colored {
            cmc += color.1;
        }
        cmc
    }
    /// If true, the card is seen in the boosters sold for the set. If false, The card is a duplicate reprint of a card in the same set. Cards seen in boosters have two numbers (bottom left of the card), separated by a slash, with the left number describing the card number in the set. Duplicate reprint cards have only one number in the bottom left of the card.
    pub fn is_in_booster(&self) -> bool {
        self.set_number as u32 <= self.set.card_count
    }
}

/// A mana cost, as seen in activated abilities and spells (creatures, instants, sorceries, etc).
#[derive(Debug)]
pub struct ManaCost {
    /// The number of mana of any color (or colorless) required to cast whatever this cost is attached to.
    pub colorless: i32,
    /// Array of a tuple of mana colors, and how many of each are required.
    pub colored: Vec<(ManaColor, i32)>,
}

/// A set of Magic the Gathering cards (example: "Portal Second Age (P02)").
#[derive(Debug)]
pub struct Set {
    /// (Usually) three letter encoding (example: "P02" for Portal Second Age) to represent the set.
    pub code: String,
    /// The full name that represents the set (example: "Portal Second Age").
    pub name: String,
    /// The number of cards in the set.
    pub card_count: u32,
    /// The date that the set was released.
    pub release_date: NaiveDate,
}

/// The price of a card.
#[derive(Debug)]
pub struct Price {
    /// The price of the card in United States Dollars (USD).
    pub usd: f32,
    /// The date the price was last fetched
    pub fetch_date: NaiveDate,
}

/// A Magic the Gathering deck.
#[derive(Debug)]
pub struct Deck {
    /// (Optional) the name of the deck.
    pub name: Option<String>,
    /// The cards in the main deck.
    pub maindeck: Vec<Card>,
    /// The cards in the sideboard.
    pub sideboard: Vec<Card>,
}

/// Incomplete list of Magic the Gathering formats.
#[derive(Debug)]
pub enum Format {
    Standard,
    Modern,
    Pioneer,
    Historic,
    Legacy,
    Vintage,
    Pauper,
    Sealed,
    BoosterDraft,
    RochesterDraft,
    Commander,
    CommanderDuel,
}

/// Card supertypes, seen on the left side of the card type, before the dash.
#[derive(Debug)]
pub enum CardType {
    BasicLand,
    Land,
    Creature,
    Artifact,
    Enchantment,
    Planeswalker,
    Instant,
    Sorcery,
}

/// Mana colors seen in Magic the Gathering, often abbreviated as WUBRG; white being W; blue being U; black being B; red being R; and green being G. Colorless is also a color.
#[derive(Debug)]
pub enum ManaColor {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

/// Card rarity.
#[derive(Debug)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    MythicRare,
    Land,
    Special,
    Token,
}
