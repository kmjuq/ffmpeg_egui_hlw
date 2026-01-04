/// 可选：截取后添加省略号（如 "超长字符串..."）
pub fn truncate_str_with_ellipsis(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let mut truncated = truncate_str_by_chars(s, max_chars);
        truncated.push_str("...");
        truncated
    }
}

/// 安全截取字符串前 n 个字符（支持中文/特殊字符）
/// 如果字符串长度不足 n，返回原字符串
pub fn truncate_str_by_chars(s: &str, max_chars: usize) -> String {
    s.chars()
        .take(max_chars) // 取前 max_chars 个字符
        .collect() // 转换为 String
}
