use std::collections::VecDeque;

const INF: i64 = i64::MAX / 2;

fn spfa(
    n: usize,             // 節點數（從 0 到 n-1）
    edges: &Vec<Vec<(usize, i64)>>, // 鄰接表：edges[u] = Vec<(v, cost)>
    source: usize         // 起點
) -> (Vec<i64>, Vec<bool>) {
    let mut dist = vec![INF; n];
    let mut in_queue = vec![false; n];
    let mut count = vec![0; n];  // 記錄每個點進隊次數，用於負環偵測
    let mut queue = VecDeque::new();

    dist[source] = 0;
    queue.push_back(source);
    in_queue[source] = true;

    while let Some(u) = queue.pop_front() {
        in_queue[u] = false;

        for &(v, w) in &edges[u] {
            if dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                if !in_queue[v] {
                    queue.push_back(v);
                    in_queue[v] = true;
                    count[v] += 1;
                    if count[v] > n {
                        // 發現負環
                        return (dist, vec![true; n]);
                    }
                }
            }
        }
    }

    (dist, vec![false; n]) // 回傳最短距離與負環標記
}
