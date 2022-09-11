use chrono::NaiveDate;

/// A Magic the Gathering card.
struct Card {
    /// The name of the card, as seen above the artwork, to the left of the casting cost.
    name: String,
    /// The supertype(s) of the card (example: "Creature", "Enchantment Creature", "Planeswalker").
    supertypes: Vec<CardType>,
    /// The subtypes of the card (example: "God", "Human Soldier", "Elemental Giant", "Aura")
    subtypes: Vec<String>,
    /// Whether the card is legendary or not (seen as the leftmost supertype of the card).
    is_legendary: bool,
    /// The rules, activated abilities, triggered abilities, keywords, etc, of the card.
    rules: Vec<String>,
    /// The flavor text (in italics) of the card, seen below its rules text.
    flavor_text: String,
    /// The power of a creature card, seen as the left number before the slash in front of its toughness. The numbers are located in the bottom right of the card.
    power: i32,
    /// The toughness of a creature card, seen as the right number after the slash after its power. The numbers are located in the bottom right of the card.
    toughness: i32,
    /// The color identities of the card (seen typically in its border).
    colors: Vec<ManaColor>,
    /// The mana costs of the card (which makes it a spell). It's seen in the top right of the card.
    mana_cost: ManaCost,
    /// The artist of the artwork in the card, seen in the bottom left of the card in very small text, to the right of a brush symbol.
    artist: String,
    /// The set of the card (the specific origin of the printing of the card).
    set: Set,
    /// The index of the card in its set.
    set_number: i32,
    /// If true, the card is seen in the booster of the set (meaning it's a new card added in the set in question). If false, the card has existed prior to the set, and the card in the set is a reprint. Cards seen in boosters have two numbers (bottom left of the card), separated by a slash, with the left number describing the card number in the set. Reprint cards have only one number in the bottom left of the card.
    in_booster: bool,
    /// The rarity of the card, seen as the color of the set symbol on the right side of the type field of the card.
    rarity: Rarity,
}

/// A mana cost, as seen in activated abilities and spells (creatures, instants, sorceries, etc).
struct ManaCost {
    /// The number of mana of any color (or colorless) required to cast whatever this cost is attached to.
    colorless: i32,
    /// Array of a tuple of mana colors, and how many of each are required.
    colored: Vec<(ManaColor, i32)>,
}

/// A set of Magic the Gathering cards (example: "Portal Second Age (P02)").
struct Set {
    /// (Usually) three letter encoding (example: "P02" for Portal Second Age) to represent the set.
    code: String,
    /// The full name that represents the set (example: "Portal Second Age").
    name: String,
    /// The number of cards in the set.
    card_count: u32,
    /// The date that the set was released.
    release_date: NaiveDate,
}

/// Card supertypes, seen on the left side of the card type, before the dash.
enum CardType {
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
enum ManaColor {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

/// Card rarity.
enum Rarity {
    Common,
    Uncommon,
    Rare,
    MythicRare,
    Land,
    Special,
    Token,
}
