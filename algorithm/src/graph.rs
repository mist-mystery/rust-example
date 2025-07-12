use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn dijkstra(
    graph: &[Vec<(usize, u32)>], // 隣接リスト: 各ノードに (to, weight) のリスト
    start: usize,
) -> Vec<u32> {
    let mut dist = vec![u32::MAX; graph.len()];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(Reverse((0, start))); // (距離, ノード)

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue; // 古い距離のエントリはスキップ
        }

        for &(v, w) in &graph[u] {
            let next_dist = d + w;
            if next_dist < dist[v] {
                dist[v] = next_dist;
                heap.push(Reverse((next_dist, v)));
            }
        }
    }

    dist
}
