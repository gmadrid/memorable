// NOTE: words should contain only ASCII letters.
//       keep the word list in alphabetical order.
pub const NOUNS: &[&str] = &[
    "agency",
    "bagpipes",
    "baldness",
    "band",
    "barnacle",
    "basket",
    "beachball",
    "bear",
    "beard",
    "boa",
    "boil",
    "bomb",
    "bones",
    "boogeyman",
    "bread",
    "brother",
    "bush",
    "butter",
    "button",
    "cannon",
    "cat",
    "chef",
    "chicanery",
    "chips",
    "clock",
    "clown",
    "coffee",
    "combustion",
    "criminal",
    "damnation",
    "dance",
    "death",
    "direction",
    "doodad",
    "doohickey",
    "dragon",
    "dump",
    "eclair",
    "elixir",
    "enchantment",
    "energy",
    "eskimo",
    "explosion",
    "eye",
    "eyeball",
    "feather",
    "fish",
    "fluid",
    "force",
    "furniture",
    "gallery",
    "gangplank",
    "garbage",
    "gargoyle",
    "gizmo",
    "goal",
    "goat",
    "goblet",
    "godmother",
    "goo",
    "goods",
    "gooseberry",
    "grandfather",
    "grandma",
    "graveyard",
    "grouch",
    "guidebook",
    "hairball",
    "hammer",
    "hand",
    "hindquarters",
    "hood",
    "hoof",
    "horseradish",
    "hose",
    "houses",
    "hyena",
    "idea",
    "idiot",
    "instrument",
    "intensity",
    "investigator",
    "investor",
    "kettle",
    "knife",
    "knuckles",
    "kumquat",
    "land",
    "laptop",
    "law",
    "lawn",
    "leg",
    "legend",
    "lemon",
    "lettuce",
    "life",
    "liquid",
    "lover",
    "machete",
    "magic",
    "male",
    "mango",
    "meat",
    "megabucks",
    "mission",
    "mistress",
    "modifier",
    "mood",
    "morality",
    "murderess",
    "newt",
    "night",
    "nitrogen",
    "object",
    "operation",
    "ornament",
    "pandemonium",
    "parliament",
    "party",
    "path",
    "pattern",
    "peer",
    "person",
    "pistol",
    "pit",
    "pond",
    "power",
    "preposition",
    "president",
    "production",
    "professional",
    "psychopath",
    "pudding",
    "quip",
    "radiation",
    "rage",
    "rainbow",
    "rattle",
    "realm",
    "resistance",
    "rocket",
    "sack",
    "salmon",
    "sandwich",
    "sasquatch",
    "scoundrel",
    "scourge",
    "sense",
    "skeleton",
    "skin",
    "skull",
    "snake",
    "squad",
    "stink",
    "stuffy",
    "style",
    "substance",
    "superstructure",
    "swampland",
    "tables",
    "tail",
    "tank",
    "teddy",
    "telephone",
    "tentacle",
    "terror",
    "therapy",
    "thunderbird",
    "toaster",
    "toilet",
    "tree",
    "tribute",
    "trout",
    "tuber",
    "tumbleweed",
    "turn",
    "underworld",
    "vandyke",
    "vegetable",
    "vision",
    "voodoo",
    "wagon",
    "war",
    "warrant",
    "wastepaper",
    "watchman",
    "werewolf",
    "whacko",
    "whammy",
    "wizardry",
    "wrestling",
    "zone",
];

#[cfg(test)]
mod test {
    use super::*;

    // A test to ensure that all words contain only letters.
    #[test]
    fn test_alpha() {
        for word in NOUNS {
            let nonalpha = word.chars().find(|ch| !ch.is_ascii_alphabetic());
            assert!(None == nonalpha, "'{}' contains non-alpha characters", word);
        }
    }

    // A test to ensure that the list is in alphabetical order. Also catches dups.
    #[test]
    fn test_order() {
        for idx in 0..(NOUNS.len() - 1) {
            let word0 = NOUNS[idx];
            let word1 = NOUNS[idx + 1];
            assert!(word0 < word1, "'{}' out of order", word1);
        }
    }
}
