#![windows_subsystem = "windows"]
use eframe::{egui, NativeOptions};
use image::load_from_memory;
use chrono::{DateTime, Local, NaiveDate, Duration};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

// 添加圖標相關的結構
#[derive(Default)]
struct IconState {
    last_remaining_days: Option<i64>,
    last_update: DateTime<Local>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct UserData {
    birthday: Option<NaiveDate>,
    life_expectancy: f32,
    name: String,
    gender: Gender,
    country: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Gender {
    Male,
    Female,
    Other,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Male
    }
}

#[derive(Default)]
struct LifeCountdownApp {
    user_data: UserData,
    current_page: Page,
    birth_year: String,
    birth_month: String,
    birth_day: String,
    show_settings: bool,
    config_path: PathBuf,
    current_quote: String,
    quote_index: usize,
    last_quote_update: DateTime<Local>,
    icon_state: IconState,
}

#[derive(Debug, PartialEq)]
enum Page {
    Setup,
    Main,
}

impl Default for Page {
    fn default() -> Self {
        Page::Setup
    }
}


impl LifeCountdownApp {
    fn load_static_icon(ctx: &egui::Context) {
        let png_bytes = include_bytes!("../assets/playstore.png");
        if let Ok(img) = load_from_memory(png_bytes) {
            let img = img.to_rgba8();
            let (w, h) = (img.width() as usize, img.height() as usize);
            let rgba = img.into_raw();
            let icon = egui::IconData { rgba, width: w as u32, height: h as u32 };
            ctx.send_viewport_cmd(egui::ViewportCommand::Icon(Some(Arc::new(icon))));
        }
    }

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();
        
        // 設置配置文件路径
        if let Some(config_dir) = dirs::config_dir() {
            app.config_path = config_dir.join("life_countdown").join("config.json");
        }
        
        // 加载用户数据
        app.load_user_data();
        
        // 設置默认值
        app.user_data.life_expectancy = 75.0;
        app.last_quote_update = Local::now();
        app.icon_state.last_update = Local::now();
        app.current_quote = "每一天都是新的開始".to_string();
        
        // 如果已有生日数据，直接进入主页面
        if app.user_data.birthday.is_some() {
            app.current_page = Page::Main;
            // 立即生成并设置icon
            // app.generate_and_set_icon(&cc.egui_ctx);
        }
        
        Self::load_static_icon(&cc.egui_ctx);
        app
    }

    // 生成圖標數據
    fn generate_icon_data(&self, remaining_days: i64) -> Vec<u8> {
        let size = 32u32;
        let mut rgba_data = vec![0u8; (size * size * 4) as usize];
        
        // 根據剩餘時間選擇顏色
        let color = if remaining_days > 10000 {
            [0, 255, 0, 255]    // 綠色
        } else if remaining_days > 5000 {
            [255, 255, 0, 255]  // 黃色
        } else if remaining_days > 1000 {
            [255, 165, 0, 255]  // 橙色
        } else {
            [255, 0, 0, 255]    // 紅色
        };
        
        let center_x = size as f32 / 2.0;
        let center_y = size as f32 / 2.0;
        let radius = size as f32 / 2.0 - 2.0;
        
        // 繪製圓形背景
        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                let pixel_index = ((y * size + x) * 4) as usize;
                
                if distance <= radius {
                    rgba_data[pixel_index] = color[0];
                    rgba_data[pixel_index + 1] = color[1];
                    rgba_data[pixel_index + 2] = color[2];
                    rgba_data[pixel_index + 3] = color[3];
                } else {
                    rgba_data[pixel_index] = 0;
                    rgba_data[pixel_index + 1] = 0;
                    rgba_data[pixel_index + 2] = 0;
                    rgba_data[pixel_index + 3] = 0;
                }
            }
        }
        
        // 繪製進度環
        if let Some(progress) = self.calculate_life_progress() {
            let progress_angle = (progress / 100.0) * 2.0 * std::f32::consts::PI;
            
            for y in 0..size {
                for x in 0..size {
                    let dx = x as f32 - center_x;
                    let dy = y as f32 - center_y;
                    let distance = (dx * dx + dy * dy).sqrt();
                    let angle = dy.atan2(dx) + std::f32::consts::PI / 2.0;
                    let normalized_angle = if angle < 0.0 { 
                        angle + 2.0 * std::f32::consts::PI 
                    } else { 
                        angle 
                    };
                    
                    if distance > radius - 3.0 && distance <= radius && normalized_angle <= progress_angle {
                        let pixel_index = ((y * size + x) * 4) as usize;
                        rgba_data[pixel_index] = 255;
                        rgba_data[pixel_index + 1] = 255;
                        rgba_data[pixel_index + 2] = 255;
                        rgba_data[pixel_index + 3] = 255;
                    }
                }
            }
        }
        
        rgba_data
    }
    
    // 生成並設置圖標
    fn generate_and_set_icon(&mut self, ctx: &egui::Context) {
        if let Some(remaining_days) = self.calculate_remaining_days() {
            let icon_data = self.generate_icon_data(remaining_days);
            
            let icon = egui::IconData {
                rgba: icon_data,
                width: 32,
                height: 32,
            };
            
            ctx.send_viewport_cmd(egui::ViewportCommand::Icon(Some(Arc::new(icon))));
            
            // 更新狀態
            self.icon_state.last_remaining_days = Some(remaining_days);
            self.icon_state.last_update = Local::now();
        }
    }
    
    // 檢查並更新圖標
    fn check_and_update_icon(&mut self, ctx: &egui::Context) {
        if let Some(remaining_days) = self.calculate_remaining_days() {
            let now = Local::now();
            
            // 檢查是否需要更新圖標（天數改變或每小時更新一次）
            let should_update = self.icon_state.last_remaining_days != Some(remaining_days) ||
                               (now - self.icon_state.last_update).num_hours() >= 1;
            
            if should_update {
                self.generate_and_set_icon(ctx);
            }
        }
    }
    
    
    fn load_user_data(&mut self) {
        if let Ok(data) = fs::read_to_string(&self.config_path) {
            if let Ok(user_data) = serde_json::from_str::<UserData>(&data) {
                self.user_data = user_data;
            }
        }
    }

    fn save_user_data(&self) {
        if let Some(parent) = self.config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        
        if let Ok(data) = serde_json::to_string_pretty(&self.user_data) {
            let _ = fs::write(&self.config_path, data);
        }
    }

    fn calculate_days_lived(&self) -> Option<i64> {
        if let Some(birthday) = self.user_data.birthday {
            let now = Local::now().date_naive();
            let days_lived = (now - birthday).num_days();
            Some(days_lived)
        } else {
            None
        }
    }

    fn calculate_remaining_days(&self) -> Option<i64> {
        if let Some(birthday) = self.user_data.birthday {
            let now = Local::now().date_naive();
            let expected_death_date = birthday + Duration::days((self.user_data.life_expectancy * 365.25) as i64);
            let remaining_days = (expected_death_date - now).num_days();
            Some(remaining_days.max(0))
        } else {
            None
        }
    }

    fn calculate_life_progress(&self) -> Option<f32> {
        if let Some(birthday) = self.user_data.birthday {
            let now = Local::now().date_naive();
            let days_lived = (now - birthday).num_days() as f32;
            let total_expected_days = self.user_data.life_expectancy * 365.25;
            Some((days_lived / total_expected_days * 100.0).min(100.0))
        } else {
            None
        }
    }

    fn get_age_in_years(&self) -> Option<f32> {
        if let Some(birthday) = self.user_data.birthday {
            let now = Local::now().date_naive();
            let years = (now - birthday).num_days() as f32 / 365.25;
            Some(years)
        } else {
            None
        }
    }

    fn update_quote(&mut self) {
        let quotes = [
            "每一天都是新的開始",
            "時間是最寶貴的資源",
            "珍惜當下，活在此刻",
            "生命的意義在於創造價值",
            "不要等待機會，要創造機會",
            "今天是你餘生的第一天",
            "時間不會回頭，所以要向前看",
            "每一刻都是禮物，這就是為什麼叫現在",
        ];
        
        let now = Local::now();
        let duration_since_last_update = now - self.last_quote_update;
        
        if duration_since_last_update.num_hours() >= 1 {
            self.quote_index = (self.quote_index + 1) % quotes.len();
            self.current_quote = quotes[self.quote_index].to_string();
            self.last_quote_update = now;
        }
    }

    fn show_setup_page(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            
            ui.heading("🎂 人生倒數計時設置");
            ui.add_space(30.0);
            
            ui.horizontal(|ui| {
                ui.label("姓名:");
                ui.text_edit_singleline(&mut self.user_data.name);
            });
            ui.add_space(10.0);
            
            ui.label("生日:");
            ui.horizontal(|ui| {
                ui.label("年:");
                ui.text_edit_singleline(&mut self.birth_year);
                ui.label("月:");
                ui.text_edit_singleline(&mut self.birth_month);
                ui.label("日:");
                ui.text_edit_singleline(&mut self.birth_day);
            });
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                ui.label("性別:");
                ui.selectable_value(&mut self.user_data.gender, Gender::Male, "男");
                ui.selectable_value(&mut self.user_data.gender, Gender::Female, "女");
                ui.selectable_value(&mut self.user_data.gender, Gender::Other, "其他");
            });
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                ui.label("預期壽命:");
                ui.add(egui::Slider::new(&mut self.user_data.life_expectancy, 60.0..=120.0)
                    .suffix(" 歲"));
            });
            ui.add_space(20.0);
            
            if ui.button("✅ 完成設置").clicked() {
                if let (Ok(year), Ok(month), Ok(day)) = (
                    self.birth_year.parse::<i32>(),
                    self.birth_month.parse::<u32>(),
                    self.birth_day.parse::<u32>(),
                ) {
                    if let Some(birthday) = NaiveDate::from_ymd_opt(year, month, day) {
                        self.user_data.birthday = Some(birthday);
                        self.current_page = Page::Main;
                        self.save_user_data();
                    }
                }
            }
        });
    }
    // Fira Code Retina

    fn show_main_page(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(remaining_days) = self.calculate_remaining_days() {
                    let name = if self.user_data.name.is_empty() { "朋友" } else { &self.user_data.name };
                    ui.label(format!("👋 {}，還剩 {} 天", name, remaining_days));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("⚙️設置").clicked() {
                            self.show_settings = !self.show_settings;
                        }
                    });
                }
            });
        });

        if self.show_settings {
            egui::SidePanel::right("settings_panel").show(ctx, |ui| {
                ui.heading("設置");
                ui.separator();
                
                ui.horizontal(|ui| {
                    ui.label("姓名:");
                    ui.text_edit_singleline(&mut self.user_data.name);
                });
                ui.add_space(10.0);
                
                ui.label("預期壽命:");
                ui.add(egui::Slider::new(&mut self.user_data.life_expectancy, 60.0..=120.0)
                    .suffix(" 歲"));
                
                ui.add_space(10.0);
                
                if ui.button("重新設定生日").clicked() {
                    self.current_page = Page::Setup;
                    self.show_settings = false;
                }
                
                ui.add_space(10.0);
                
                if ui.button("保存設置").clicked() {
                    self.save_user_data();
                    self.show_settings = false;
                }
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                
                if let Some(remaining_days) = self.calculate_remaining_days() {
                    ui.add_space(30.0);
                    ui.label(
                        egui::RichText::new(format!("{}", remaining_days))
                            .size(80.0)
                            .color(egui::Color32::from_rgb(100, 149, 237))
                            .strong()
                    );
                    
                    ui.label(
                        egui::RichText::new("天")
                            .size(40.0)
                            .color(egui::Color32::from_rgb(100, 149, 237))
                    );
                    
                    ui.add_space(10.0);
                    ui.label("剩餘時間");
                    ui.add_space(30.0);
                    
                    if let Some(progress) = self.calculate_life_progress() {
                        ui.label(format!("人生進度: {:.1}%", progress));
                        let progress_bar = egui::ProgressBar::new(progress / 100.0)
                            .text(format!("{:.1}%", progress));
                        ui.add(progress_bar);
                    }
                    
                    ui.add_space(20.0);
                    
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.label("📊 統計訊息");
                            ui.separator();
                            
                            if let Some(age) = self.get_age_in_years() {
                                ui.label(format!("當前年齡: {:.1} 歲", age));
                            }
                            
                            if let Some(days_lived) = self.calculate_days_lived() {
                                ui.label(format!("已度過: {} 天", days_lived));
                            }
                            
                            ui.label(format!("預期壽命: {:.0} 歲", self.user_data.life_expectancy));
                            
                            let years = remaining_days / 365;
                            let months = (remaining_days % 365) / 30;
                            let days = remaining_days % 30;
                            
                            ui.label(format!("約為: {} 年 {} 月 {} 天", years, months, days));
                        });
                    });
                    
                    ui.add_space(30.0);
                    
                    ui.group(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label("💭 今日思考");
                            ui.separator();
                            ui.label(
                                egui::RichText::new(&self.current_quote)
                                    .size(16.0)
                                    .color(egui::Color32::from_rgb(105, 105, 105))
                                    .italics()
                            );
                        });
                    });
                    
                    if remaining_days <= 365 {
                        ui.add_space(20.0);
                        ui.colored_label(
                            egui::Color32::from_rgb(255, 69, 0),
                            "⚠️ 珍惜時光"
                        );
                    } else if remaining_days <= 1825 {
                        ui.add_space(20.0);
                        ui.colored_label(
                            egui::Color32::from_rgb(255, 140, 0),
                            "⏰ 時間不多"
                        );
                    }
                }
            });
        });
    }
}

impl eframe::App for LifeCountdownApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 首先檢查並更新圖標（在GUI渲染之前）
        self.check_and_update_icon(ctx);
        
        // 更新名言
        self.update_quote();
        
        // 設置字體
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert("my_font".to_owned(),
            egui::FontData::from_static(include_bytes!("../../jf-openhuninn-2.1.ttf"))
        );
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
            .insert(0, "my_font".to_owned());
        fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
            .push("my_font".to_owned());
        ctx.set_fonts(fonts);
        
        ctx.set_visuals(egui::Visuals {
            window_rounding: egui::Rounding::same(10.0),
            window_shadow: egui::epaint::Shadow {
                offset: Default::default(), // Vec2::ZERO
                blur: 10.0,
                spread: 0.0,
                color:  egui::Color32::from_black_alpha(50),  // ← 新增這行
            },
            ..egui::Visuals::dark()
        });

        // 渲染頁面
        match self.current_page {
            Page::Setup => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.show_setup_page(ui);
                });
            }
            Page::Main => {
                self.show_main_page(ctx);
            }
        }
        
        // 每分鐘刷新一次
        ctx.request_repaint_after(std::time::Duration::from_secs(60));
    }
}
fn main() -> Result<(), eframe::Error> {
    // 1) 编译时加载 playstore.png
    let png_bytes = include_bytes!("../assets/playstore.png");
    let img = load_from_memory(png_bytes)
        .expect("Failed to load playstore.png")
        .to_rgba8();
    let icon = egui::IconData {
        rgba: img.clone().into_raw(),
        width: img.clone().width() as u32,
        height: img.clone().height() as u32,
    };

    // 2) 在 viewport builder 里设置 icon
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0])
            .with_title("人生倒數計時 - Life Countdown")
            .with_icon(icon),  // ← 这里
        ..Default::default()
    };

    // 3) 启动
    eframe::run_native(
        "人生倒數計時",
        options,
        Box::new(|cc| Ok(Box::new(LifeCountdownApp::new(cc)))),
    )
}