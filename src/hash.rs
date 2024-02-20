use md5::compute;

pub fn urlshortner(url: &str) -> String {
    let digest = compute(url);
    let mut short_url = String::new();
    for byte in digest.iter().take(3) {
        short_url.push_str(&format!("{:02x}", byte));
    }
    short_url
}