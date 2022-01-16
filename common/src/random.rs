use rand::Rng;

pub fn code() -> String {
    let mut rng = rand::thread_rng();
    rng.gen::<u64>();
    rng.gen_range(100000..=999999).to_string()
}
