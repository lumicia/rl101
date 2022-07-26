pub fn can_place_flowers(flowerbed: &[i32], n: i32) -> bool {
    if flowerbed.len() == 0 {
        return false;
    }

    let mut num = 0;
    let mut zeroes = 1;
    let len = flowerbed.len();

    for i in 0..len {
        if flowerbed[i] == 0 {
            zeroes += 1;
        } else {
            num += (zeroes - 1) / 2;
            zeroes = 0;
        }
    }

    zeroes += 1;
    num += (zeroes - 1) / 2;

    num >= n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_place_flowers() {
        let flowerbed = [1, 0, 0, 0, 1];
        let n = 1;
        assert!(can_place_flowers(&flowerbed, n));
    }

    #[test]
    fn test_can_place_flowers_two() {
        let flowerbed = [1, 0, 0, 0, 1];
        let n = 2;
        assert!(!can_place_flowers(&flowerbed, n));
    }
}
