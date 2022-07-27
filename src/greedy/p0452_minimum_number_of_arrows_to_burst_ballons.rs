pub fn find_min_arrow_shots(points: &mut [[i32; 2]]) -> i32 {
    let n = points.len();
    if n == 0 {
        return 0;
    }
    points.sort_by(|i, j| i[1].cmp(&j[1]));

    let mut right = points[0][1];
    let mut count = 1;

    for i in 0..n {
        if points[i][0] > right {
            count += 1;
            right = points[i][1];
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_arrow_shots() {
        let mut points = vec![[10, 16], [2, 8], [1, 6], [7, 12]];
        assert_eq!(2, find_min_arrow_shots(&mut points));
    }

    #[test]
    fn test_find_min_arrow_shots_two() {
        let mut points = vec![[1, 2], [3, 4], [5, 6], [7, 8]];
        assert_eq!(4, find_min_arrow_shots(&mut points));
    }

    #[test]
    fn test_find_min_arrow_shots_three() {
        let mut points = vec![[1, 2], [2, 3], [3, 4], [4, 5]];
        assert_eq!(2, find_min_arrow_shots(&mut points));
    }

    #[test]
    fn test_find_min_arrow_shots_four() {
        let mut points = vec![[9, 12], [1, 10], [4, 11], [8, 12], [3, 9], [6, 9], [6, 7]];
        assert_eq!(2, find_min_arrow_shots(&mut points));
    }
}
