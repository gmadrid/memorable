use memorable;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    for _ in 0..10 {
        eprintln!("{}", memorable::memorable_id(&mut rng));
    }
}
