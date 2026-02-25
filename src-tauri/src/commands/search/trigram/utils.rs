pub fn split_path(path: String) -> Vec<String> {
    // 2 spaces prefix and 1 space suffix
    let padded_path = format!("  {} ", path);
    let lower_padded = padded_path.to_ascii_lowercase();
    lower_padded
        .chars()
        .collect::<Vec<char>>()
        .windows(3)
        .map(|e| e.iter().collect::<String>())
        .collect()
}
