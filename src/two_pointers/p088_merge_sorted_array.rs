pub fn merge(nums1: &mut Vec<i32>, nums2: &mut Vec<i32>, m: i32, n: i32) {
    let mut pos: usize = (m + n - 1) as usize;
    let mut m: usize = (m - 1) as usize;
    let mut n: usize = (n - 1) as usize;

    while m >= 0 && n >= 0 {
        if nums1[m] >= nums2[n] {
            nums1[pos] = nums1[m];
            if m == 0 {
                break;
            }
            m -= 1;
            pos -= 1;
        } else {
            nums1[pos] = nums2[n];
            if n == 0 {
                break;
            }
            n -= 1;
            pos -= 1;
        }
    }

    while n >= 0 {
        nums1[pos] = nums2[n];
        if n == 0 {
            break;
        }
        n -= 1;
        pos -= 1;
    }
}

pub fn merge2(nums1: &mut Vec<u32>, nums2: &mut Vec<u32>, m: i32, n: i32) {
    let mut m: usize = m as usize;
    let _n = n;
    let mut i: usize = 0;

    nums2.iter().for_each(|num| {
        while i < nums1.len() {
            if *num < nums1[i] || i == m {
                nums1.insert(i, *num);
                nums1.pop();
                i += 1;
                m += 1;
                break;
            }
            i += 1;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        let m = 3;
        let n = 3;
        merge(&mut nums1, &mut nums2, m, n);
        assert_eq!(vec![1, 2, 2, 3, 5, 6], nums1);
    }

    #[test]
    fn test_merge2() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        let m = 3;
        let n = 3;
        merge2(&mut nums1, &mut nums2, m, n);
        assert_eq!(vec![1, 2, 2, 3, 5, 6], nums1);
    }
}
