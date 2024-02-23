use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn generate_random_executable_name(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect::<String>()
        + ".exe"
}
