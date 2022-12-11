// NOTE: words should contain only ASCII letters.
//       keep the word list in alphabetical order.
pub const ADJECTIVES: &[&str] = &[
    "abnormal",
    "abusive",
    "alcoholic",
    "alphabetical",
    "angry",
    "arborial",
    "architectural",
    "arrogant",
    "atomic",
    "bearded",
    "black",
    "cloven",
    "clumsy",
    "cocky",
    "confused",
    "controlled",
    "costumed",
    "covert",
    "cowardly",
    "dangling",
    "dead",
    "deep",
    "disgusting",
    "disturbing",
    "domesticated",
    "drunken",
    "ducky",
    "eternal",
    "evil",
    "fighting",
    "filthy",
    "flirtatious",
    "fluffy",
    "frisky",
    "frozen",
    "great",
    "greedy",
    "hairless",
    "harsh",
    "hateful",
    "haunting",
    "hideous",
    "hilarious",
    "holy",
    "hot",
    "hyperactive",
    "idiotic",
    "impressive",
    "indecent",
    "indelible",
    "insane",
    "insta",
    "internal",
    "international",
    "least",
    "loose",
    "loquacious",
    "magical",
    "maniacal",
    "massive",
    "medicated",
    "metal",
    "naked",
    "nuclear",
    "offensive",
    "peripheral",
    "personal",
    "poopy",
    "professional",
    "raw",
    "sad",
    "sadistic",
    "scientific",
    "sexy",
    "shaky",
    "sick",
    "sinister",
    "slimy",
    "slippery",
    "sticky",
    "stubborn",
    "supernatural",
    "tactful",
    "talking",
    "territorial",
    "theatrical",
    "twisted",
    "ultraviolet",
    "ungodly",
    "unholy",
    "unnoticed",
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

    // A test to ensure that the list is in alphabetical order.
    #[test]
    fn test_order() {
        for idx in 0..(ADJECTIVES.len() - 1) {
            let word0 = ADJECTIVES[idx];
            let word1 = ADJECTIVES[idx + 1];
            assert!(word0 < word1, "'{}' out of order", word1);
        }
    }
}
