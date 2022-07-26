use std::cmp::max;

pub fn candy(ratings: &mut [u32]) -> u32 {
    let n = ratings.len();
    if n < 2 {
        return n as u32;
    }

    ratings.sort();
    let mut candies: u32 = 1;
    let mut total: u32 = 0;

    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            candies += 1;
            total += candies;
        } else {
            total += candies;
        }
    }

    total
}

pub fn candy2(ratings: &[u32]) -> u32 {
    let n = ratings.len();
    if n < 2 {
        return n as u32;
    }

    let mut num = vec![1; n];

    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            num[i] = num[i - 1] + 1;
        }
    }

    for i in (1..n).rev() {
        if ratings[i] < ratings[i - 1] {
            num[i - 1] = max(num[i - 1], num[i] + 1);
        }
    }

    num.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candy() {
        let mut ratings = vec![1, 0, 2];
        assert_eq!(5, candy(&mut ratings));
    }

    #[test]
    fn test_candy2() {
        let mut ratings = vec![1, 0, 2];
        assert_eq!(5, candy2(&mut ratings));
    }
}
