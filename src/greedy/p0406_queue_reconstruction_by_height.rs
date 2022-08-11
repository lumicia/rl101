use std::iter::repeat;

pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut people = people;
    let n = people.len();
    let mut result: Vec<Vec<i32>> = repeat(vec![0, 0]).take(n).collect();
    // people.sort_by(|i, j| {
    //     if i[0] == j[0] {
    //         i[1].cmp(&j[1])
    //     } else {
    //         j[0].cmp(&i[0])
    //     }
    // });
    people.sort_by_key(|k| (-(k[0]), k[1]));

    for cur in people {
        let i = cur[1] as usize;
        result.insert(i, cur);
    }

    result[0..n].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reconstruct_queue() {
        let people = vec![
            vec![7, 0],
            vec![4, 4],
            vec![7, 1],
            vec![5, 0],
            vec![6, 1],
            vec![5, 2],
        ];
        assert_eq!(
            vec![
                vec![5, 0],
                vec![7, 0],
                vec![5, 2],
                vec![6, 1],
                vec![4, 4],
                vec![7, 1]
            ],
            reconstruct_queue(people)
        );
    }

    #[test]
    fn test_reconstruct_queue_two() {
        let people = vec![
            vec![6, 0],
            vec![5, 0],
            vec![4, 0],
            vec![3, 2],
            vec![2, 2],
            vec![1, 4],
        ];
        assert_eq!(
            vec![
                vec![4, 0],
                vec![5, 0],
                vec![2, 2],
                vec![3, 2],
                vec![1, 4],
                vec![6, 0]
            ],
            reconstruct_queue(people)
        );
    }
}
