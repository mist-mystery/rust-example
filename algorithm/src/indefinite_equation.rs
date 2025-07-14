pub fn solve(coeffs: &[usize], target: usize) -> Vec<Vec<usize>> {
    let mut solutions = Vec::new();
    let mut current = Vec::with_capacity(coeffs.len());

    fn dfs(
        coeffs: &[usize],
        target: usize,
        index: usize,
        current: &mut Vec<usize>,
        solutions: &mut Vec<Vec<usize>>,
    ) {
        if index == coeffs.len() - 1 {
            // 最後の変数：直接計算してチェック
            let coeff = coeffs[index];
            if target > 0 && target % coeff == 0 {
                let x = target / coeff;
                if x >= 1 {
                    current.push(x);
                    solutions.push(current.clone());
                    current.pop();
                }
            }
            return;
        }

        let coeff = coeffs[index];
        // x_i は 1 以上 target/coeff まで
        for x in 1..=target / coeff {
            let used = coeff * x;
            if used > target {
                break;
            }
            current.push(x);
            dfs(coeffs, target - used, index + 1, current, solutions);
            current.pop();
        }
    }

    if !coeffs.is_empty() {
        dfs(coeffs, target, 0, &mut current, &mut solutions);
    }

    solutions
}

pub fn solve_ordered(coeffs: &[usize], target: usize) -> Vec<Vec<usize>> {
    let mut solutions = Vec::new();
    let mut current = Vec::with_capacity(coeffs.len());

    fn dfs(
        coeffs: &[usize],
        target: usize,
        index: usize,
        min_value: usize, // x_i >= min_value
        current: &mut Vec<usize>,
        solutions: &mut Vec<Vec<usize>>,
    ) {
        if index == coeffs.len() - 1 {
            let coeff = coeffs[index];
            if target % coeff == 0 {
                let x = target / coeff;
                if x >= min_value {
                    current.push(x);
                    solutions.push(current.clone());
                    current.pop();
                }
            }
            return;
        }

        let coeff = coeffs[index];
        // 探索は min_value から（非減少列を保証）
        for x in min_value..=target / coeff {
            let used = coeff * x;
            if used > target {
                break;
            }
            current.push(x);
            dfs(coeffs, target - used, index + 1, x, current, solutions); // 次は x 以上
            current.pop();
        }
    }

    if !coeffs.is_empty() {
        dfs(coeffs, target, 0, 1, &mut current, &mut solutions);
    }

    solutions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_solutions_two() {
        // すべての係数が互いに素
        let solutions = solve(&[2, 3], 16);
        assert_eq!(solutions, [[2, 4], [5, 2]]);

        // 両辺を2倍した場合（最大公約数が1でない）
        let solutions = solve(&[4, 6], 32);
        assert_eq!(solutions, [[2, 4], [5, 2]]);

        // 解なし
        let solutions = solve(&[5, 7], 8);
        assert_eq!(solutions, [] as [[usize; 0]; 0]);
    }

    #[test]
    fn test_find_solutions_three() {
        // すべての係数が互いに素
        let solutions = solve(&[3, 4, 5], 30);
        assert_eq!(
            solutions,
            [[1, 3, 3], [2, 1, 4], [3, 4, 1], [4, 2, 2], [7, 1, 1]]
        );

        // 係数が互いに素でない場合
        let solutions = solve(&[2, 3, 4], 20);
        assert_eq!(solutions, [[1, 2, 3], [2, 4, 1], [3, 2, 2], [5, 2, 1]]);
    }

    #[test]
    fn test_solve_ordered_three() {
        // すべての係数が互いに素
        let solutions = solve_ordered(&[3, 4, 5], 30);
        assert_eq!(solutions, [[1, 3, 3]]);

        // 係数が互いに素でない場合
        let solutions = solve_ordered(&[2, 3, 4], 20);
        assert_eq!(solutions, [[1, 2, 3]]);
    }

    #[test]
    fn test_find_solutions_five() {
        let solutions = solve_ordered(&[669, 596, 485, 403, 361], 123_450);
        solutions.iter().for_each(|s| {
            assert_eq!(
                [669, 596, 485, 403, 361]
                    .iter()
                    .zip(s)
                    .fold(0, |acc, (c, x)| acc + c * x),
                123_450
            );
        });
    }
}
