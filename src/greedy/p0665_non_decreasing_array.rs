pub fn check_possibility(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    let mut count = 0;
    let n = nums.len();

    for i in 1..n {
        if nums[i] < nums[i - 1] {
            count += 1;
            if count > 1 {
                return false;
            }
            if i < 2 || nums[i - 2] <= nums[i] {
                nums[i - 1] = nums[i];
            } else {
                nums[i] = nums[i - 1];
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_possibility() {
        let nums = vec![4, 2, 3];
        assert!(check_possibility(nums));
    }

    #[test]
    fn test_check_possibility_two() {
        let nums = vec![4, 2, 1];
        assert!(!check_possibility(nums));
    }

    #[test]
    fn test_check_possibility_three() {
        let nums = vec![4, 7, 3, 9];
        assert!(check_possibility(nums));
    }

    #[test]
    fn test_check_possibility_four() {
        let nums = vec![1, 7, 3, 4];
        assert!(check_possibility(nums));
    }

    #[test]
    fn test_check_possibility_five() {
        let nums = vec![3, 4, 2, 3];
        assert!(!check_possibility(nums));
    }
}
