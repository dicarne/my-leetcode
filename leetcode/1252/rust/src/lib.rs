struct Solution {}

impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut matrix: Vec<Vec<i32>> = vec![];
        for mi in 0..m {
            let mut line = vec![];
            for ni in 0..n {
                line.push(0);
            }
            matrix.push(line);
        }
        for it in indices.iter() {
            let ri = it[0];
            let ci = it[1];
            for i in 0..n {
                matrix[ri as usize][i as usize] += 1;
            }
            for i in 0..m {
                matrix[i as usize][ci as usize] += 1;
            }

        }
        let mut sum = 0;
        for mi in 0..m {
            for ni in 0..n {
                if matrix[mi as usize][ni as usize] % 2 == 1 {
                    sum += 1;
                }
            }
        }
        return sum;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let a1 = Solution::odd_cells(2, 3, vec![vec![0,1],vec![1,1]]);
        assert_eq!(a1, 6);
    }
}
