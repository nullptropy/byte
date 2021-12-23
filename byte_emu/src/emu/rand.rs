// from https://matklad.github.io/2023/01/04/on-random-numbers.html
pub fn random_seed() -> u64 {
    std::hash::Hasher::finish(&std::hash::BuildHasher::build_hasher(
        &std::collections::hash_map::RandomState::new(),
    ))
}

pub fn random_numbers(seed: u32) -> impl Iterator<Item = u32> {
    let mut random = seed;

    std::iter::repeat_with(move || {
        random ^= random << 13;
        random ^= random >> 17;
        random ^= random << 5;
        random
    })
}
