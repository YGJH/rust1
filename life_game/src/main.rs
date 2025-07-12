use eframe::egui;
// use std::collections::HashSet;
use crate::egui::FontFamily;
use std::fmt;
// use egui::FontFamily;
use crate::egui::FontData;
use crate::egui::FontDefinitions;


#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Dead = 0,
    Alive = 1,
}
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match *self {
            Cell::Alive => "⬜",
            Cell::Dead => "⬛"
        };
        write!(f, "{}", symbol)
    }
}

pub struct GameOfLifeApp {
    universe: Universe,
    is_running: bool,
    speed: f32,
    timer: f32,
    grid_size: usize,
    cell_size: f32,
    generation: u32,
    drawing: bool,
    erasing: bool,
}

pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Universe {
        let cells = vec![Cell::Dead; width * height];
        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn clear(&mut self) {
        self.cells.fill(Cell::Dead);
    }
    pub fn set_gosper_glider_gun(&mut self, offset_row: usize, offset_col: usize) {
        // Gosper Glider Gun 的相對座標（經典版本）
        let coords: [(usize, usize); 36] = [
            (5,1), (5,2), (6,1), (6,2),
            (3,13), (3,14),
            (4,12), (4,16),
            (5,11), (5,17),
            (6,11), (6,15), (6,17), (6,18),
            (7,11), (7,17),
            (8,12), (8,16),
            (9,13), (9,14),
            (1,25), (2,23), (2,25),
            (3,21), (3,22),
            (4,21), (4,22),
            (5,21), (5,22),
            (6,23), (6,25),
            (7,25),
            (3,35), (3,36),
            (4,35), (4,36),
        ];
        for (r, c) in coords.iter().cloned() {
            let row = offset_row + r;
            let col = offset_col + c;
            if row < self.height && col < self.width {
                let idx = self.get_index(row, col);
                self.cells[idx] = Cell::Alive;
            }
        }
    }

    pub fn randomize(&mut self) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        std::time::SystemTime::now().hash(&mut hasher);
        let seed = hasher.finish();
        
        for (i, cell) in self.cells.iter_mut().enumerate() {
            let x = (seed.wrapping_mul(i as u64 + 1)) % 100;
            *cell = if x < 30 { Cell::Alive } else { Cell::Dead };
        }
    }

    pub fn set_cells(&mut self, cells: &[(usize, usize)]) {
        for (row, col) in cells.iter().cloned() {
            if row < self.height && col < self.width {
                let idx = self.get_index(row, col);
                self.cells[idx] = Cell::Alive;
            }
        }
    }

    pub fn toggle_cell(&mut self, row: usize, col: usize) {
        if row < self.height && col < self.width {

            let idx = self.get_index(row, col);
            self.cells[idx] = match self.cells[idx] {
                Cell::Alive => Cell::Dead,
                Cell::Dead => Cell::Alive,
            };
        }
    }

    pub fn set_cell(&mut self, row: usize, col: usize, state: Cell) {
        if row < self.height && col < self.width {
            let idx = self.get_index(row, col);
            self.cells[idx] = state;
        }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Cell {
        if row < self.height && col < self.width {
            let idx = self.get_index(row, col);
            self.cells[idx]
        } else {
            Cell::Dead
        }
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    fn live_neighbor_count(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        
        for delta_row in -1..=1 {
            for delta_col in -1..=1 {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                
                let neighbor_row = row as i32 + delta_row;
                let neighbor_col = col as i32 + delta_col;
                
                if neighbor_row >= 0 && neighbor_row < self.height as i32 &&
                   neighbor_col >= 0 && neighbor_col < self.width as i32 {
                    let idx = self.get_index(neighbor_row as usize, neighbor_col as usize);
                    count += self.cells[idx] as u8;
                }
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn count_alive(&self) -> usize {
        self.cells.iter().filter(|&&cell| cell == Cell::Alive).count()
    }
}

impl Default for GameOfLifeApp {
    fn default() -> Self {
        let grid_size = 100;
        let mut universe = Universe::new(grid_size, grid_size);
        
        // 設置一些初始模式
        universe.set_cells(&[
            // 滑翔機
            (1, 2), (2, 3), (3, 1), (3, 2), (3, 3),
            // 振盪器
            (10, 10), (10, 11), (10, 12),
            // 另一個滑翔機
            (20, 20), (20, 21), (20, 22), (21, 20), (22, 21),
        ]);

        Self {
            universe,
            is_running: false,
            speed: 10.0,
            timer: 0.0,
            grid_size,
            cell_size: 8.0,
            generation: 0,
            drawing: false,
            erasing: false,
        }
    }
}

impl eframe::App for GameOfLifeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 頂部控制面板
        let mut fonts = FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters):
        fonts.font_data.insert("my_font".to_owned(),
            FontData::from_static(include_bytes!("../../jf-openhuninn-2.1.ttf"))
        
        );

        // Put my font first (highest priority):
        fonts.families.get_mut(&FontFamily::Proportional).unwrap()
            .insert(0, "my_font".to_owned());

        // Put my font as last fallback for monospace:
        fonts.families.get_mut(&FontFamily::Monospace).unwrap()
            .push("my_font".to_owned());
        ctx.set_fonts(fonts);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button(if self.is_running { "⏸ 暫停" } else { "▶ 開始" }).clicked() {
                    self.is_running = !self.is_running;
                }
                
                if ui.button("⏹ 停止").clicked() {
                    self.is_running = false;
                    self.generation = 0;
                }
                
                if ui.button("⏭ 下一步").clicked() {
                    self.universe.tick();
                    self.generation += 1;
                }
                
                if ui.button("🗑 清空").clicked() {
                    self.universe.clear();
                    self.generation = 0;
                }
                
                if ui.button("🎲 隨機").clicked() {
                    self.universe.randomize();
                    self.generation = 0;
                }
                
                ui.separator();
                
                ui.label("速度:");
                ui.add(egui::Slider::new(&mut self.speed, 1.0..=256.0).suffix(" FPS"));
                
                ui.separator();
                
                ui.label("細胞大小:");
                ui.add(egui::Slider::new(&mut self.cell_size, 2.0..=20.0));
                
                ui.separator();
                
                ui.label(format!("世代: {}", self.generation));
                ui.label(format!("活細胞: {}", self.universe.count_alive()));
            });
        });

        // 左側模式面板
        egui::SidePanel::left("patterns_panel").show(ctx, |ui| {
            ui.heading("經典模式");
            
            if ui.button("滑翔機").clicked() {
                self.universe.clear();
                self.universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
                self.generation = 0;
            }
            
            if ui.button("振盪器").clicked() {
                self.universe.clear();
                self.universe.set_cells(&[(10, 10), (10, 11), (10, 12)]);
                self.generation = 0;
            }
            
            if ui.button("蟾蜍").clicked() {
                self.universe.clear();
                self.universe.set_cells(&[
                    (10, 11), (10, 12), (10, 13),
                    (11, 10), (11, 11), (11, 12)
                ]);
                self.generation = 0;
            }
            if ui.button("滑翔機槍").clicked() {
                self.universe.clear();
                self.universe.set_gosper_glider_gun(0, 0);
                self.generation = 0;
           }

            if ui.button("信標").clicked() {
                self.universe.clear();
                self.universe.set_cells(&[
                    (10, 10), (10, 11), (11, 10), (11, 11),
                    (12, 12), (12, 13), (13, 12), (13, 13)
                ]);
                self.generation = 0;
            }
            
            if ui.button("太空船").clicked() {
                self.universe.clear();
                self.universe.set_cells(&[
                    (10, 11), (10, 14), (11, 15), (12, 11), (12, 15),
                    (13, 12), (13, 13), (13, 14), (13, 15)
                ]);
                self.generation = 0;
            }
            
            ui.separator();
            ui.label("使用說明:");
            ui.label("• 點擊細胞切換狀態");
            ui.label("• 拖拽繪製細胞");
            ui.label("• 按住 Shift 擦除");
        });

        // 主要遊戲區域
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_rect = ui.available_rect_before_wrap();
            let (response, painter) = ui.allocate_painter(available_rect.size(), egui::Sense::click_and_drag());
            
            // 處理鼠標輸入
            if response.drag_started() || response.dragged() {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let relative_pos = pointer_pos - response.rect.min;
                    let col = (relative_pos.x / self.cell_size) as usize;
                    let row = (relative_pos.y / self.cell_size) as usize;
                    
                    if response.drag_started() {
                        self.universe.toggle_cell(row, col);
                    } else if response.dragged() {
                        let is_shift_held = ui.input(|i| i.modifiers.shift);
                        let state = if is_shift_held { Cell::Dead } else { Cell::Alive };
                        self.universe.set_cell(row, col, state);
                    }
                }
            }
            
            // 繪製網格
            let grid_color = egui::Color32::from_gray(100);
            let alive_color = egui::Color32::from_rgb(0, 255, 0);
            let dead_color = egui::Color32::from_rgb(20, 20, 20);
            
            // 繪製細胞
            for row in 0..self.universe.height {
                for col in 0..self.universe.width {
                    let cell = self.universe.get_cell(row, col);
                    let x = response.rect.min.x + col as f32 * self.cell_size;
                    let y = response.rect.min.y + row as f32 * self.cell_size;
                    
                    let rect = egui::Rect::from_min_size(
                        egui::pos2(x, y),
                        egui::vec2(self.cell_size, self.cell_size)
                    );
                    
                    let color = match cell {
                        Cell::Alive => alive_color,
                        Cell::Dead => dead_color,
                    };
                    
                    painter.rect_filled(rect, 0.0, color);
                    painter.rect_stroke(rect, 0.0, egui::Stroke::new(0.5, grid_color));
                }
            }
        });

        // 自動更新邏輯
        if self.is_running {
            self.timer += ctx.input(|i: &egui::InputState| i.unstable_dt);
            let target_interval = 1.0 / self.speed;
            
            if self.timer >= target_interval {
                self.universe.tick();
                self.generation += 1;
                self.timer = 0.0;
            }
        }
        
        // 持續重繪
        ctx.request_repaint();
    }
}


fn main() -> Result<(), eframe::Error> {
    let mut fonts = FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert("my_font".to_owned(),
        FontData::from_static(include_bytes!("../jf-openhuninn-2.1.ttf"))
    
    );

    // Put my font first (highest priority):
    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts.families.get_mut(&FontFamily::Monospace).unwrap()
        .push("my_font".to_owned());


    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("康威生命遊戲 - Conway's Game of Life"),
        ..Default::default()
    };
    
    eframe::run_native(
        "康威生命遊戲",
        options,
        Box::new(|_cc| Box::new(GameOfLifeApp::default())),
    )
}