use std::cell::RefCell;

#[cfg(target_arch = "wasm32")]
thread_local! {
    static SEED_U32: RefCell<u64> = RefCell::new(0x12345678abcdef);
    static SEED_F64: RefCell<u64> = RefCell::new(0xabcdef12345678);
}

#[cfg(target_arch = "wasm32")]
pub fn random_u32() -> u32 {
    SEED_U32.with(|s| {
        let mut seed = s.borrow_mut();
        *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        ((*seed >> 24) & 0xFFFFFFFF) as u32
    })
}

#[cfg(target_arch = "wasm32")]
pub fn random_f64() -> f64 {
    SEED_F64.with(|s| {
        let mut seed = s.borrow_mut();
        *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        ((*seed >> 40) as f64) / ((1u64 << 24) as f64)
    })
}

#[cfg(not(target_arch = "wasm32"))]
pub fn random_u32() -> u32 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.r#gen::<u32>()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn random_f64() -> f64 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.r#gen()
}