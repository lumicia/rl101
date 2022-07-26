pub fn candy(ratings: &mut Vec<u32>) -> u32 {
    ratings.sort();
    let mut candies: u32 = 1;
    let mut total: u32 = 0;
    let n = ratings.len();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candy() {
        let mut ratings = vec![1, 0, 2];
        assert_eq!(5, candy(&mut ratings));
    }
}
