use rand::{thread_rng, Rng};

mod adjectives;
mod nouns;

pub fn memorable_id_simple() -> String {
    let mut rng = thread_rng();
    memorable_id(&mut rng)
}

pub fn memorable_id(mut rng: &mut impl Rng) -> String {
    let adj = rand_word(&mut rng, adjectives::ADJECTIVES);
    let noun = rand_word(&mut rng, nouns::NOUNS);

    format!("{}-{}", adj, noun)
}

fn rand_word(rng: &mut impl Rng, lst: &[&'static str]) -> &'static str {
    let n = rng.gen_range(0, lst.len());
    lst[n]
}
