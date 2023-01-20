use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn format_rtt(rtt: f64) -> String {
    let rtt = rtt / 1000.0;
    format!("{:.1} ms", rtt)
}

pub fn generate_random_string(length: usize) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    rand_string
}
