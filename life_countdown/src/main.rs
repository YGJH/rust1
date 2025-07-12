use eframe::egui;
use chrono::{DateTime, Local, NaiveDate, Duration};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use egui::FontFamily;
use egui::FontData;
use egui::FontDefinitions;
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
    show_celebration: bool,
    celebration_message: String,
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
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
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
        
        // 如果已有生日数据，直接进入主页面
        if app.user_data.birthday.is_some() {
            app.current_page = Page::Main;
        }
        
        app
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
        let now = Local::now();
        let duration_since_last_update = now - self.last_quote_update;
        
        // 每小时更换一次名言
        if duration_since_last_update.num_hours() >= 1 {
            self.last_quote_update = now;
        }
    }

    fn show_setup_page(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            
            // 标题
            ui.heading("🎂 設置");
            ui.add_space(30.0);
            
            // 姓名输入
            ui.horizontal(|ui| {
                ui.label("姓名:");
                ui.text_edit_singleline(&mut self.user_data.name);
            });
            ui.add_space(10.0);
            
            // 生日输入
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
            
            // 性别选择
            ui.horizontal(|ui| {
                ui.label("性別:");
                ui.selectable_value(&mut self.user_data.gender, Gender::Male, "男");
                ui.selectable_value(&mut self.user_data.gender, Gender::Female, "女");
                ui.selectable_value(&mut self.user_data.gender, Gender::Other, "其他");
            });
            ui.add_space(10.0);
            
            // 预期寿命
            ui.horizontal(|ui| {
                ui.label("預期壽命:");
                ui.add(egui::Slider::new(&mut self.user_data.life_expectancy, 60.0..=120.0)
                    .suffix(" 歲"));
            });
            ui.add_space(20.0);
            
            // 确认按钮
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

    fn show_main_page(&mut self, ctx: &egui::Context) {
        // 顶部菜单栏
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(remaining_days) = self.calculate_remaining_days() {
                    ui.label(format!("👋 剩餘, {} 天", remaining_days));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("⚙️設置").clicked() {
                            self.show_settings = !self.show_settings;
                        }
                    });

                }
            });
        });

        // 設置面板
        if self.show_settings {
            egui::SidePanel::right("settings_panel").show(ctx, |ui| {
                ui.heading("設置");
                ui.separator();
                
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

        // 主要内容区域
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                
                // 主要倒计时显示
                if let Some(remaining_days) = self.calculate_remaining_days() {
                    // 大数字显示
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
                    
                    // 进度条
                    if let Some(progress) = self.calculate_life_progress() {
                        ui.label(format!("人生進度: {:.1}%", progress));
                        let progress_bar = egui::ProgressBar::new(progress / 100.0)
                            .text(format!("{:.1}%", progress));
                        ui.add(progress_bar);
                    }
                    
                    ui.add_space(20.0);
                    
                    // 统计信息
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
                            
                            // 转换为年月日
                            let years = remaining_days / 365;
                            let months = (remaining_days % 365) / 30;
                            let days = remaining_days % 30;
                            
                            ui.label(format!("約為: {} 年 {} 月 {} 天", years, months, days));
                        });
                    });
                    
                    ui.add_space(30.0);
                    
                    // 励志名言
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
                    
                    // 特殊提醒
                    if remaining_days <= 365 {
                        ui.add_space(20.0);
                        ui.colored_label(
                            egui::Color32::from_rgb(255, 69, 0),
                            "⚠️ 珍惜時光"
                        );
                    } else if remaining_days <= 1825 { // 5年
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
        // 更新名言
        self.update_quote();
        
        // 設置全局样式
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

        ctx.set_visuals(egui::Visuals {
            window_rounding: egui::Rounding::same(10.0),
            window_shadow: egui::epaint::Shadow {
                offset: egui::vec2(5.0, 10.0),
                blur: 20.0,
                spread: 0.0,
                color: egui::Color32::from_black_alpha(50),
            },
            ..egui::Visuals::dark()
        });

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
        
        // 每分钟刷新一次
        ctx.request_repaint_after(std::time::Duration::from_secs(60));
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("人生倒數計時 - Life Countdown")
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "人生倒數計時",
        options,
        Box::new(|cc| Ok(Box::new(LifeCountdownApp::new(cc)))),
    )
}
