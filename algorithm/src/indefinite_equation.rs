/// 一時不定方程式の自然数解を求める。
///
/// 与えられた係数のリストと定数項(target)に対して、
/// 方程式 `c_0 * x_0 + c_1 * x_1 + ... + c_n * x_n = target` の自然数解を深さ優先探索で求める。
/// ただし、`c_i` は係数、`x_i` は変数であり、すべての `x_i` は自然数（1以上）
pub fn solve(coeffs: &[usize], target: usize) -> Vec<Vec<usize>> {
    let mut solutions = Vec::new();
    let mut current = Vec::with_capacity(coeffs.len());

    if !coeffs.is_empty() {
        dfs(coeffs, target, 0, 1, false, &mut current, &mut solutions);
    }

    solutions
}

/// 一次不定方程式の自然数解を求める。
///
/// 与えられた係数のリストと定数項(target)に対して
/// 方程式 `c_0 * x_0 + c_1 * x_1 + ... + c_n * x_n = target` の自然数解を深さ優先探索で求める。
/// ただし、`c_i` は係数、`x_i` は変数であり、すべての `x_i` は自然数（1以上）、かつ `x_i >= x_{i-1}` を満たす。
/// 解が昇順でない場合を枝狩りするため、実行速度が速い。
pub fn solve_asc(coeffs: &[usize], target: usize) -> Vec<Vec<usize>> {
    let mut solutions = Vec::new();
    let mut current = Vec::with_capacity(coeffs.len());

    if !coeffs.is_empty() {
        dfs(coeffs, target, 0, 1, true, &mut current, &mut solutions);
    }

    solutions
}

/// 深さ優先探索を用いて、一次不定方程式の与えられた係数と定数項に対する自然数解を探索する。
///
/// # Arguments
/// * `coeffs` - 係数のリスト
/// * `target` - 方程式の右辺の値
/// * `index` - 現在の(探索中の)係数のインデックス
/// * `min_value` - 現在の変数 `x_i` が満たすべき最小値
/// * `asc` - 解を昇順にするかどうかのフラグ
/// * `current` - 現在の解のリスト（探索中の解）
/// * `solutions` - 見つかった解のリスト（最終的な結果を格納していく）
fn dfs(
    coeffs: &[usize],
    target: usize,
    index: usize,
    min_value: usize, // x_i >= min_value
    asc: bool,
    current: &mut Vec<usize>,
    solutions: &mut Vec<Vec<usize>>,
) {
    let coeff = coeffs[index];

    // 一番最後の係数の場合、target が coeff の倍数である必要がある。
    if index == coeffs.len() - 1 {
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

    // x_i の値を min_value から順番に1ずつ増やしていって探索する。
    for x in min_value..=target / coeff {
        let used = coeff * x;

        // もし現在の変数の値が target を超えたら、探索を打ち切る。
        if used > target {
            break;
        }

        current.push(x);
        dfs(
            coeffs, // 係数列
            target - used,
            index + 1,               // 次の変数へ
            if asc { x } else { 1 }, // 次の変数 x_{i+1} は >= min_value
            asc,                     // 解を昇順にするか。true ならば x_i >= x_{i-1} を保証
            current,
            solutions,
        );
        current.pop();
    }
}

pub fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut factor_count = vec![0; limit + 1];
    factor_count[1] = 0;

    for i in 2..=limit.isqrt() {
        for j in (i * i..=limit).step_by(i) {
            factor_count[j] += 1;
        }
    }
    factor_count
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
        let solutions = solve_asc(&[3, 4, 5], 30);
        assert_eq!(solutions, [[1, 3, 3]]);

        // 係数が互いに素でない場合
        let solutions = solve_asc(&[2, 3, 4], 20);
        assert_eq!(solutions, [[1, 2, 3]]);
    }

    #[test]
    #[ignore]
    fn test_solve_five() {
        let coeffs = [669, 596, 485, 403, 361];
        let solutions = solve(&coeffs, 100_000);
        solutions.iter().for_each(|s| {
            assert_eq!(
                coeffs.iter().zip(s).fold(0, |acc, (c, x)| acc + c * x),
                100_000
            );
        });
        assert_eq!(solutions.len(), 140_795);
    }

    #[test]
    fn test_solve_asc_five() {
        let coeffs = [669, 596, 485, 403, 361];
        let solutions = solve_asc(&coeffs, 278_790);
        assert_eq!(solutions.len(), 159_432);
    }
}
