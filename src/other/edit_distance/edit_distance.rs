

use std::cmp::min;

pub fn edit_distance(s1: &str, s2: &str) -> i32 {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    dp(&s1_chars, &s2_chars, s1_chars.len() - 1, s2_chars.len() - 1) as i32
}

fn dp(s1: &Vec<char>, s2: &Vec<char>, i: usize, j: usize) -> usize {
    if i == 0 { return j + 1; }
    if j == 0 { return i + 1; }

    return if s1[i] == s2[j] {
        dp(s1, s2, i - 1, j - 1)
    } else {
        min(
            dp(s1, s2, i, j - 1) + 1,
            min(
                dp(s1, s2, i - 1, j) + 1,
                dp(s1, s2, i - 1, j - 1) + 1,
            ),
        )
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_edit_distance() {
        let s1 = "abc";
        let s2 = "dcdf";
        println!("dis {}", edit_distance(s1, s2))
    }
}