use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuadTree {
    level: u8,
    nw: Option<Rc<QuadTree>>,
    ne: Option<Rc<QuadTree>>,
    sw: Option<Rc<QuadTree>>,
    se: Option<Rc<QuadTree>>,
    alive: bool,
}

pub struct HashLifeUniverse {
    root: Rc<QuadTree>,
    cache: HashMap<(Rc<QuadTree>, u32), Rc<QuadTree>>,
    node_cache: HashMap<(u8, Option<Rc<QuadTree>>, Option<Rc<QuadTree>>, Option<Rc<QuadTree>>, Option<Rc<QuadTree>>), Rc<QuadTree>>,
    generation: u64,
}

impl QuadTree {
    pub fn new_cell(alive: bool) -> Rc<Self> {
        Rc::new(QuadTree {
            level: 0,
            nw: None,
            ne: None,
            sw: None,
            se: None,
            alive,
        })
    }

    pub fn new_node(level: u8, nw: Rc<QuadTree>, ne: Rc<QuadTree>, sw: Rc<QuadTree>, se: Rc<QuadTree>) -> Rc<Self> {
        Rc::new(QuadTree {
            level,
            nw: Some(nw),
            ne: Some(ne),
            sw: Some(sw),
            se: Some(se),
            alive: false,
        })
    }

    pub fn size(&self) -> u32 {
        1 << self.level
    }

    pub fn population(&self) -> u64 {
        if self.level == 0 {
            if self.alive { 1 } else { 0 }
        } else {
            self.nw.as_ref().unwrap().population() +
            self.ne.as_ref().unwrap().population() +
            self.sw.as_ref().unwrap().population() +
            self.se.as_ref().unwrap().population()
        }
    }
}

impl HashLifeUniverse {
    pub fn new() -> Self {
        let dead_cell = QuadTree::new_cell(false);
        let root = QuadTree::new_node(2, dead_cell.clone(), dead_cell.clone(), dead_cell.clone(), dead_cell);
        
        HashLifeUniverse {
            root,
            cache: HashMap::new(),
            node_cache: HashMap::new(),
            generation: 0,
        }
    }

    fn get_node(&mut self, level: u8, nw: Rc<QuadTree>, ne: Rc<QuadTree>, sw: Rc<QuadTree>, se: Rc<QuadTree>) -> Rc<QuadTree> {
        let key = (level, Some(nw.clone()), Some(ne.clone()), Some(sw.clone()), Some(se.clone()));
        
        if let Some(node) = self.node_cache.get(&key) {
            node.clone()
        } else {
            let node = QuadTree::new_node(level, nw, ne, sw, se);
            self.node_cache.insert(key, node.clone());
            node
        }
    }

    fn expand(&mut self) {
        let level = self.root.level + 1;
        let dead_quad = self.empty_quad(self.root.level);
        
        // 先提取出當前根節點的四個子節點
        let old_nw = self.root.nw.as_ref().unwrap().clone();
        let old_ne = self.root.ne.as_ref().unwrap().clone();
        let old_sw = self.root.sw.as_ref().unwrap().clone();
        let old_se = self.root.se.as_ref().unwrap().clone();
        
        // 然後分別創建新的子節點
        let new_nw = self.get_node(level - 1, dead_quad.clone(), dead_quad.clone(), dead_quad.clone(), old_nw);
        let new_ne = self.get_node(level - 1, dead_quad.clone(), dead_quad.clone(), old_ne, dead_quad.clone());
        let new_sw = self.get_node(level - 1, dead_quad.clone(), old_sw, dead_quad.clone(), dead_quad.clone());
        let new_se = self.get_node(level - 1, old_se, dead_quad.clone(), dead_quad.clone(), dead_quad.clone());
        
        // 最後創建新的根節點
        self.root = self.get_node(level, new_nw, new_ne, new_sw, new_se);
    }
    fn empty_quad(&mut self, level: u8) -> Rc<QuadTree> {
        if level == 0 {
            QuadTree::new_cell(false)
        } else {
            let sub = self.empty_quad(level - 1);
            self.get_node(level, sub.clone(), sub.clone(), sub.clone(), sub)
        }
    }

    fn next_generation(&mut self, node: &Rc<QuadTree>, steps: u32) -> Rc<QuadTree> {
        if steps == 0 {
            return node.clone();
        }

        let cache_key = (node.clone(), steps);
        if let Some(cached) = self.cache.get(&cache_key) {
            return cached.clone();
        }

        let result = if node.level == 2 {
            self.next_gen_4x4(node)
        } else {
            // 簡化版本 - 只處理一步
            let nw = self.next_generation(node.nw.as_ref().unwrap(), 1);
            let ne = self.next_generation(node.ne.as_ref().unwrap(), 1);
            let sw = self.next_generation(node.sw.as_ref().unwrap(), 1);
            let se = self.next_generation(node.se.as_ref().unwrap(), 1);
            
            self.get_node(node.level - 1, nw, ne, sw, se)
        };

        self.cache.insert(cache_key, result.clone());
        result
    }

    fn next_gen_4x4(&mut self, node: &Rc<QuadTree>) -> Rc<QuadTree> {
        // 簡化版本 - 直接返回當前狀態
        let nw = node.nw.as_ref().unwrap();
        let ne = node.ne.as_ref().unwrap();
        let sw = node.sw.as_ref().unwrap();
        let se = node.se.as_ref().unwrap();
        
        self.get_node(1, nw.clone(), ne.clone(), sw.clone(), se.clone())
    }

    pub fn set_cell(&mut self, x: i32, y: i32, alive: bool) {
        let mut root = self.root.clone();
        let size = root.size() as i32;
        let half_size = size / 2;

        while x < -half_size || x >= half_size || y < -half_size || y >= half_size {
            self.expand();
            root = self.root.clone();
        }
        
        let new_root = self.set_cell_recursive(&root, x, y, alive, 0, 0);
        self.root = new_root;
    }

    fn set_cell_recursive(&mut self, node: &Rc<QuadTree>, x: i32, y: i32, alive: bool, node_x: i32, node_y: i32) -> Rc<QuadTree> {
        if node.level == 0 {
            return QuadTree::new_cell(alive);
        }
        
        let half_size = (node.size() / 2) as i32;
        let (new_nw, new_ne, new_sw, new_se) = if x < node_x {
            if y < node_y {
                (self.set_cell_recursive(node.nw.as_ref().unwrap(), x, y, alive, node_x - half_size, node_y - half_size),
                 node.ne.as_ref().unwrap().clone(),
                 node.sw.as_ref().unwrap().clone(),
                 node.se.as_ref().unwrap().clone())
            } else {
                (node.nw.as_ref().unwrap().clone(),
                 node.ne.as_ref().unwrap().clone(),
                 self.set_cell_recursive(node.sw.as_ref().unwrap(), x, y, alive, node_x - half_size, node_y + half_size),
                 node.se.as_ref().unwrap().clone())
            }
        } else {
            if y < node_y {
                (node.nw.as_ref().unwrap().clone(),
                 self.set_cell_recursive(node.ne.as_ref().unwrap(), x, y, alive, node_x + half_size, node_y - half_size),
                 node.sw.as_ref().unwrap().clone(),
                 node.se.as_ref().unwrap().clone())
            } else {
                (node.nw.as_ref().unwrap().clone(),
                 node.ne.as_ref().unwrap().clone(),
                 node.sw.as_ref().unwrap().clone(),
                 self.set_cell_recursive(node.se.as_ref().unwrap(), x, y, alive, node_x + half_size, node_y + half_size))
            }
        };
        
        self.get_node(node.level, new_nw, new_ne, new_sw, new_se)
    }

    pub fn get_cell(&self, x: i32, y: i32) -> bool {
        let size = self.root.size() as i32;
        let half_size = size / 2;
        
        if x < -half_size || x >= half_size || y < -half_size || y >= half_size {
            return false;
        }
        
        self.get_cell_recursive(&self.root, x, y, 0, 0)
    }

    fn get_cell_recursive(&self, node: &Rc<QuadTree>, x: i32, y: i32, node_x: i32, node_y: i32) -> bool {
        if node.level == 0 {
            return node.alive;
        }
        
        let half_size = (node.size() / 2) as i32;
        
        if x < node_x {
            if y < node_y {
                self.get_cell_recursive(node.nw.as_ref().unwrap(), x, y, node_x - half_size, node_y - half_size)
            } else {
                self.get_cell_recursive(node.sw.as_ref().unwrap(), x, y, node_x - half_size, node_y + half_size)
            }
        } else {
            if y < node_y {
                self.get_cell_recursive(node.ne.as_ref().unwrap(), x, y, node_x + half_size, node_y - half_size)
            } else {
                self.get_cell_recursive(node.se.as_ref().unwrap(), x, y, node_x + half_size, node_y + half_size)
            }
        }
    }

    pub fn step(&mut self) {
        while self.root.level < 3 {
            self.expand();
        }
        
        let root = self.root.clone();
        let steps = 1;
        let new_root = self.next_generation(&root, steps);
        self.root = new_root;
        self.generation += steps as u64;
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn population(&self) -> u64 {
        self.root.population()
    }

    pub fn clear(&mut self) {
        let dead_cell = QuadTree::new_cell(false);
        self.root = QuadTree::new_node(2, dead_cell.clone(), dead_cell.clone(), dead_cell.clone(), dead_cell);
        self.generation = 0;
        self.cache.clear();
    }
}