// 字符串遍历

/// 字符串不能用index直接遍历.需要转成chars才可以

fn index(s:&str) -> char {
    let chars: Vec<char> = s.chars().collect();
    chars[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_index() {
        let s = "abc";
        println!("char {}", index(s));
    }
}
