pub fn two_sum(numbers: &[i32], target: i32) -> Vec<i32> {
    let (mut left, mut right) = (0, numbers.len() - 1);
    let mut sum = 0;

    while left < right {
        sum = numbers[left] + numbers[right];
        if sum == target {
            break;
        }
        if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }

    vec![(left + 1) as i32, (right + 1) as i32]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let a = vec![2, 7, 11, 15];
        assert_eq!(vec![1, 2], two_sum(&a, 9));
    }
}
