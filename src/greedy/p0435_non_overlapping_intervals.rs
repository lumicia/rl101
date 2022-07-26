pub fn erase_overlap_intervals(intervals: &mut [[u32; 2]]) -> u32 {
    if intervals.is_empty() {
        return 0;
    }

    let n = intervals.len();
    intervals.sort_by(|i, j| i[1].cmp(&j[1]));

    let mut removed = 0;
    let mut prev = intervals[0][1];

    for i in 1..n {
        if intervals[i][0] < prev {
            removed += 1;
        } else {
            prev = intervals[i][1];
        }
    }

    removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erase_overlap_intervals() {
        let mut intervals = vec![[1, 2], [2, 4], [1, 3]];
        assert_eq!(1, erase_overlap_intervals(&mut intervals));
    }
}
