pub fn passGen() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            // This is safe because `idx` is in range of `CHARSET`
            char::from(unsafe { *CHARSET.get_unchecked(idx) })
        })
        .collect();

    println!("{:?}", password);
}