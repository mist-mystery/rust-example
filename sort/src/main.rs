use std::time;

use sort::{exchange_sort, insertion_sort, merge_sort, selection_sort};

fn main() {
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
