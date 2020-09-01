pub const ADJECTIVES: &[&str] = &[
    "abnormal",
    "abusive",
    "alcoholic",
    "alphabetical",
    "angry",
    "arborial",
    "arrogant",
    "bearded",
    "clumsy",
    "cocky",
    "confused",
    "costumed",
    "cowardly",
    "dead",
    "deep",
    "disgusting",
    "disturbing",
    "domesticated",
    "drunken",
    "ducky",
    "fighting",
    "filthy",
    "flirtatious",
    "fluffy",
    "frisky",
    "frozen",
    "greedy",
    "hairless",
    "harsh",
    "hateful",
    "haunting",
    "hideous",
    "hilarious",
    "hot",
    "hyperactive",
    "idiotic",
    "impressive",
    "indecent",
    "indelible",
    "insane",
    "insta",
    "loquacious",
    "magical",
    "maniacal",
    "massive",
    "medicated",
    "metal",
    "naked",
    "offensive",
    "poopy",
    "sadistic",
    "sexy",
    "shaky",
    "sick",
    "sinister",
    "slimy",
    "slippery",
    "sticky",
    "stubborn",
    "tactful",
    "talking",
    "territorial",
    "twisted",
    "ungodly",
    "unholy",
    "useless",
    "vengeful",
    "violent",
    "wild",
    "zippy",
];

#[cfg(test)]
mod test {
    use super::*;

    // A test to ensure that all words contain only letters.
    #[test]
    fn test_alpha() {
        for word in ADJECTIVES {
            let nonalpha = word.chars().find(|ch| !ch.is_ascii_alphabetic());
            assert_eq!(None, nonalpha, "'{}' contains non-alpha characters", word);
        }
    }

    #[test]
    fn test_order() {
        for idx in 0..(ADJECTIVES.len() - 1) {
            let word0 = ADJECTIVES[idx];
            let word1 = ADJECTIVES[idx + 1];
            assert!(word0 < word1, "'{}' out of order", word1);
        }
    }
}
