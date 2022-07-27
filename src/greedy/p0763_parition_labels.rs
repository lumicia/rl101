use std::cmp::max;

pub fn partition_labels(s: &str) -> Vec<i32> {
    let mut letters = [0; 26];
    let mut result = vec![];
    let mut left = 0;
    let mut right = 0;

    for (i, c) in s.chars().enumerate() {
        letters[c as usize - 'a' as usize] = i;
    }

    for (i, c) in s.chars().enumerate() {
        right = max(right, letters[c as usize - 'a' as usize]);
        if i == right {
            result.push((right - left + 1) as i32);
            left = right + 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_labels() {
        let s = "ababcbacadefegdehijhklij";
        assert_eq!(vec![9, 7, 8], partition_labels(s));
    }

    #[test]
    fn test_partition_labels_two() {
        let s = "eccbbbbdec";
        assert_eq!(vec![10], partition_labels(s));
    }
}
