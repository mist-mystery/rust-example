use std::time;

use algorithm::{
    graph,
    sort::{self, exchange_sort, insertion_sort, merge_sort, selection_sort},
};

fn main() {
    exec_sort();
    exec_search();
}

fn exec_sort() {
    slow_sort();
    faster_sort();
}

fn exec_search() {
    let graph = vec![
        vec![(1, 4), (2, 1)], // 0 → 1 (4), 0 → 2 (1)
        vec![(3, 1)],         // 1 → 3 (1)
        vec![(1, 2), (3, 5)], // 2 → 1 (2), 2 → 3 (5)
        vec![],               // 3 has no outgoing edges
    ];

    let start = 0;
    let dist = graph::dijkstra(&graph, start);

    for (i, d) in dist.iter().enumerate() {
        println!("dist[{}] = {}", i, d); // dist[0] = 0, dist[1] = 3, dist[2] = 1, dist[3] = 4
    }
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
