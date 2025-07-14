use std::time;

use algorithm::{
    indefinite_equation,
    sort::{self, exchange_sort, insertion_sort, merge_sort, selection_sort},
};

fn main() {
    solve_indefinite_equation(278_790, 5);
    exec_sort();
}

fn exec_sort() {
    slow_sort();
    faster_sort();
}

fn slow_sort() {
    let v = sort::make_random_vector(20_000);

    let mut v_bubble = v.clone();
    let now = time::Instant::now();
    exchange_sort::bubble(&mut v_bubble);
    println!("Bubble: {:?}", now.elapsed());

    let mut v_shaker = v.clone();
    let now = time::Instant::now();
    exchange_sort::shaker(&mut v_shaker);
    println!("Shaker: {:?}", now.elapsed());

    let mut v_selection = v.clone();
    let now = time::Instant::now();
    selection_sort::selection(&mut v_selection);
    println!("Selection: {:?}", now.elapsed());

    let mut v_insertion = v.clone();
    let now = time::Instant::now();
    insertion_sort::insertion(&mut v_insertion);
    println!("Insertion: {:?}", now.elapsed());
}

fn faster_sort() {
    let v = sort::make_random_vector(1_000_000);

    let mut v_quick = v.clone();
    let now = time::Instant::now();
    exchange_sort::quick(&mut v_quick);
    println!("Quick: {:?}", now.elapsed());

    let mut v_heap = v.clone();
    let now = time::Instant::now();
    selection_sort::heap(&mut v_heap);
    println!("Heap: {:?}", now.elapsed());

    let mut v_merge = v.clone();
    let now = time::Instant::now();
    let sorted_merge = merge_sort::merge(&mut v_merge);
    println!(
        "Merge (compare, swap): ({}, {}), {:?}",
        sorted_merge.compare_count,
        sorted_merge.swap_count,
        now.elapsed()
    );
}

fn solve_indefinite_equation(target: usize, factor_count: usize) {
    let coeffs = [669, 596, 485, 403, 361];
    let solutions = indefinite_equation::solve_asc(&coeffs, target);
    let factor_counts = indefinite_equation::sieve_of_eratosthenes(target / coeffs[4]);

    let mut solution_variances = solutions
        .into_iter()
        .map(|solves| {
            let mean = solves.iter().map(|x| *x as f64).sum::<f64>() / solves.len() as f64;
            let variance = solves
                .iter()
                .map(|x| (*x as f64 - mean).powi(2))
                .sum::<f64>()
                / solves.len() as f64;

            (solves, variance)
        })
        // 全ての解の因数が4つ以上ある解のみを抽出
        .filter(|(solves, _)| solves.iter().all(|&c| factor_counts[c] >= factor_count))
        .collect::<Vec<_>>();

    solution_variances.sort_by_key(|(_, variance)| *variance as usize);
    let filtered: Vec<Vec<_>> = solution_variances
        .into_iter()
        .map(|(solves, _)| solves)
        .collect();

    println!("{:?}", filtered);
}
