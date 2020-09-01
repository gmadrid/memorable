// NOTE: words should contain only ASCII letters.
//       keep the word list in alphabetical order.
pub const NOUNS: &[&str] = &[
    "band",
    "beachball",
    "boil",
    "brother",
    "button",
    "chef",
    "clown",
    "coffee",
    "criminal",
    "dragon",
    "eskimo",
    "eyeball",
    "fluid",
    "goal",
    "goo",
    "grandma",
    "graveyard",
    "grouch",
    "guidebook",
    "hairball",
    "hammer",
    "hose",
    "hyena",
    "idea",
    "idiot",
    "intensity",
    "investigator",
    "investor",
    "kettle",
    "knife",
    "kumquat",
    "laptop",
    "leg",
    "legend",
    "lettuce",
    "lover",
    "machete",
    "mistress",
    "mood",
    "morality",
    "newt",
    "party",
    "person",
    "pistol",
    "pond",
    "president",
    "rage",
    "rattle",
    "sandwich",
    "scoundrel",
    "scourge",
    "skeleton",
    "skull",
    "squad",
    "stuffy",
    "tank",
    "telephone",
    "therapy",
    "toaster",
    "toilet",
    "tuber",
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

    // A test to ensure that the list is in alphabetical order.
    #[test]
    fn test_order() {
        for idx in 0..(NOUNS.len() - 1) {
            let word0 = NOUNS[idx];
            let word1 = NOUNS[idx + 1];
            assert!(word0 < word1, "'{}' out of order", word1);
        }
    }
}
