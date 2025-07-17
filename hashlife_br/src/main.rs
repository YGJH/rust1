use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// 四叉樹節點，level 表示區域邊長為 2^level
#[derive(Debug)]
struct Node {
    level: u32,
    population: u32,
    nw: Option<Rc<Node>>,
    ne: Option<Rc<Node>>,
    sw: Option<Rc<Node>>,
    se: Option<Rc<Node>>,
    /// 快取此節點跳躍 2^(level-2) 步後的結果
    result: RefCell<Option<Rc<Node>>>,
}

/// 用原始指標與 level 作為雜湊鍵，實現 hash-consing
#[derive(Eq, PartialEq)]
struct NodeKey {
    level: u32,
    nw: *const Node,
    ne: *const Node,
    sw: *const Node,
    se: *const Node,
}
impl Hash for NodeKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.level.hash(state);
        (self.nw as usize).hash(state);
        (self.ne as usize).hash(state);
        (self.sw as usize).hash(state);
        (self.se as usize).hash(state);
    }
}

/// Hashlife 主體，保存節點快取與根目錄
struct Universe {
    cache: RefCell<HashMap<NodeKey, Rc<Node>>>,
    leaf0: Rc<Node>,  
    leaf1: Rc<Node>,  
}

impl Universe {
    /// 初始化兩個 leaf 節點並建立快取
    fn new() -> Self {
        let leaf0 = Rc::new(Node {
            level: 0,
            population: 0,
            nw: None, ne: None, sw: None, se: None,
            result: RefCell::new(None),
        });
        let leaf1 = Rc::new(Node {
            level: 0,
            population: 1,
            nw: None, ne: None, sw: None, se: None,
            result: RefCell::new(None),
        });
        Universe {
            cache: RefCell::new(HashMap::new()),
            leaf0,
            leaf1,
        }
    }

    /// 取得 leaf 節點
    fn leaf(&self, alive: bool) -> Rc<Node> {
        if alive { self.leaf1.clone() } else { self.leaf0.clone() }
    }

    /// 建構或共用一個 level +1 的四叉樹節點
    fn node(&self, nw: Rc<Node>, ne: Rc<Node>, sw: Rc<Node>, se: Rc<Node>) -> Rc<Node> {
        let level = nw.level + 1;
        let population = nw.population + ne.population + sw.population + se.population;
        let key = NodeKey {
            level,
            nw: Rc::as_ptr(&nw),
            ne: Rc::as_ptr(&ne),
            sw: Rc::as_ptr(&sw),
            se: Rc::as_ptr(&se),
        };
        if let Some(existing) = self.cache.borrow().get(&key) {
            return existing.clone();
        }
        let new_node = Rc::new(Node {
            level,
            population,
            nw: Some(nw.clone()),
            ne: Some(ne.clone()),
            sw: Some(sw.clone()),
            se: Some(se.clone()),
            result: RefCell::new(None),
        });
        self.cache.borrow_mut().insert(key, new_node.clone());
        new_node
    }

    /// 讀取節點中 (x,y) 處的生死，x,y 範圍為 [0, 2^level)
    fn get_cell(&self, node: &Rc<Node>, x: usize, y: usize) -> bool {
        if node.level == 0 {
            return node.population > 0;
        }
        let half = 1 << (node.level - 1);
        if x < half {
            if y < half {
                self.get_cell(node.nw.as_ref().unwrap(), x, y)
            } else {
                self.get_cell(node.sw.as_ref().unwrap(), x, y - half)
            }
        } else {
            if y < half {
                self.get_cell(node.ne.as_ref().unwrap(), x - half, y)
            } else {
                self.get_cell(node.se.as_ref().unwrap(), x - half, y - half)
            }
        }
    }

    /// 基底：對 level=2 的 4x4 區塊直接模擬 1 步，並回傳中心 2x2 節點
    fn compute_level2(&self, node: &Rc<Node>) -> Rc<Node> {
        let mut grid = [[false; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                grid[i][j] = self.get_cell(node, i, j);
            }
        }
        let mut centers = [[false; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                let mut count = 0;
                for di in 0..3 {
                    for dj in 0..3 {
                        if di == 1 && dj == 1 { continue; }
                        if grid[i + di][j + dj] { count += 1; }
                    }
                }
                centers[i][j] = if grid[i + 1][j + 1] {
                    count == 2 || count == 3
                } else {
                    count == 3
                };
            }
        }
        self.node(
            self.leaf(centers[0][0]),
            self.leaf(centers[0][1]),
            self.leaf(centers[1][0]),
            self.leaf(centers[1][1]),
        )
    }

    /// 提取中心子節點（level-1）用於遞歸
    fn centered_subnode(&self, node: &Rc<Node>) -> Rc<Node> {
        self.node(
            node.nw.as_ref().unwrap().se.as_ref().unwrap().clone(),
            node.ne.as_ref().unwrap().sw.as_ref().unwrap().clone(),
            node.sw.as_ref().unwrap().ne.as_ref().unwrap().clone(),
            node.se.as_ref().unwrap().nw.as_ref().unwrap().clone(),
        )
    }

    /// 遞歸計算 hash life：跳躍 2^(level-2) 步後的結果
    fn next_generation(&self, node: &Rc<Node>) -> Rc<Node> {
        if let Some(cached) = node.result.borrow().clone() {
            return cached;
        }
        let res = if node.level == 2 {
            self.compute_level2(node)
        } else {
            let a = self.next_generation(&self.centered_subnode(node.nw.as_ref().unwrap()));
            let b = self.next_generation(&self.centered_subnode(node));
            let c = self.next_generation(&self.centered_subnode(node));
            let d = self.next_generation(&self.centered_subnode(node.se.as_ref().unwrap()));
            self.node(a, b, c, d)
        };
        node.result.replace(Some(res.clone()));
        res
    }
}

fn main() {
    let uni = Universe::new();
    let inner = uni.node(
        uni.node(uni.leaf(false), uni.leaf(true), uni.leaf(true), uni.leaf(false)),
        uni.node(uni.leaf(false), uni.leaf(true), uni.leaf(true), uni.leaf(false)),
        uni.node(uni.leaf(false), uni.leaf(true), uni.leaf(true), uni.leaf(false)),
        uni.node(uni.leaf(false), uni.leaf(true), uni.leaf(true), uni.leaf(false)),
    );
    let next = uni.next_generation(&inner);
    println!("Next population: {}", next.population);
}
