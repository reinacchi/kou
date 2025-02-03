pub fn bits_to_mb(bytes: u64) -> f64 {
    return bytes as f64 / (1024.0 * 1024.0);
}
