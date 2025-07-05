use std::time;

use sort::{exchange_sort, insertion_sort, merge_sort, selection_sort};

fn main() {
    slow_sort();
    faster_sort();
}

fn slow_sort() {
    let v = sort::make_random_vector(10_000);

    let mut v_bubble = v.clone();
    let now = time::Instant::now();
    let sorted_bubble = exchange_sort::bubble(&mut v_bubble);
    println!(
        "Bubble (compare, swap): ({}, {}), {:?}",
        sorted_bubble.compare_count,
        sorted_bubble.swap_count,
        now.elapsed()
    );

    let mut v_selection = v.clone();
    let now = time::Instant::now();
    let sorted_selection = selection_sort::selection(&mut v_selection);
    println!(
        "Selection (compare, swap): ({}, {}), {:?}",
        sorted_selection.compare_count,
        sorted_selection.swap_count,
        now.elapsed()
    );

    let mut v_insertion = v.clone();
    let now = time::Instant::now();
    let sorted_insertion = insertion_sort::insertion(&mut v_insertion);
    println!(
        "Insertion (compare, swap): ({}, {}), {:?}",
        sorted_insertion.compare_count,
        sorted_insertion.swap_count,
        now.elapsed()
    );
}

fn faster_sort() {
    let v = sort::make_random_vector(1_000_000);

    let mut v_quick = v.clone();
    let now = time::Instant::now();
    let sorted_quick = exchange_sort::quick(&mut v_quick);
    println!(
        "Quick (compare, swap): ({}, {}), {:?}",
        sorted_quick.compare_count,
        sorted_quick.swap_count,
        now.elapsed()
    );

    let mut v_heap = v.clone();
    let now = time::Instant::now();
    let sorted_heap = selection_sort::heap(&mut v_heap);
    println!(
        "Heap (compare, swap): ({}, {}), {:?}",
        sorted_heap.compare_count,
        sorted_heap.swap_count,
        now.elapsed()
    );

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
