use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i64,
}

pub struct Dinic {
    size: usize,
    graph: Vec<Vec<Edge>>,
    level: Vec<i32>,
    iter: Vec<usize>,
}

impl Dinic {
    pub fn new(mut n: usize) -> Self {
        n+=1;
        Self {
            size: n,
            graph: vec![vec![]; n],
            level: vec![-1; n],
            iter: vec![0; n],
        }
    }

    /// 加邊（從 from 到 to，容量為 cap）
    pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
        let rev_from = self.graph[to].len();
        let rev_to = self.graph[from].len();
        self.graph[from].push(Edge { to, rev: rev_from, cap });
        self.graph[to].push(Edge { to: from, rev: rev_to, cap: 0 }); // 反向邊容量 0
    }

    /// BFS 分層圖
    fn bfs(&mut self, s: usize, t: usize) {
        self.level = vec![-1; self.size];
        let mut queue = VecDeque::new();
        self.level[s] = 0;
        queue.push_back(s);
        while let Some(v) = queue.pop_front() {
            for e in &self.graph[v] {
                if e.cap > 0 && self.level[e.to] < 0 {
                    self.level[e.to] = self.level[v] + 1;
                    queue.push_back(e.to);
                }
            }
        }
    }

    /// DFS 尋找增廣路
    fn dfs(&mut self, v: usize, t: usize, up_to: i64) -> i64 {
        if v == t {
            return up_to;
        }

        let lv = self.level[v];
        while self.iter[v] < self.graph[v].len() {
            let i = self.iter[v];
            let (e_to, e_cap, _) = {
                let e = &self.graph[v][i];
                (e.to, e.cap, e.rev)
            }; // 这里 e 的借用结束
            if e_cap > 0 && self.level[e_to] == lv + 1 {
                let d = self.dfs(e_to, t, up_to.min(e_cap));
                if d > 0 {
                    // 更新正向與反向邊
                    self.graph[v][i].cap -= d;
                    let rev = self.graph[v][i].rev;
                    self.graph[e_to][rev].cap += d;
                    return d;
                }
            }
            self.iter[v] += 1;
        }

        0
    }

    /// 主函數：求最大流
    pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
        let mut flow = 0;
        loop {
            self.bfs(s, t);
            if self.level[t] < 0 {
                break;
            }
            self.iter = vec![0; self.size];
            loop {
                let f = self.dfs(s, t, i64::MAX);
                if f == 0 {
                    break;
                }
                flow += f;
            }
        }
        flow
    }
}
