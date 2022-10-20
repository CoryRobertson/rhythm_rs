use eframe::emath::Align2;
use eframe::epaint::{Color32, FontId, Pos2, Rect, Rounding, Stroke};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {

    #[serde(skip)]
    time: wasm_timer::SystemTime,

    #[serde(skip)]
    button_click_time: wasm_timer::SystemTime,

    #[serde(skip)]
    prev_difference: f64,

    #[serde(skip)]
    frames: i32,

    #[serde(skip)]
    bpm: f64,

    #[serde(skip)]
    previous_bpm_ratings: Vec<f32>,

    bpm_target: i32,
    epsilon: i32,
    prev_bpm_store_count: usize,
    
    displaying_indicator: bool,

}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            time: wasm_timer::SystemTime::now(),
            frames: 0,
            button_click_time: wasm_timer::SystemTime::now(),
            prev_difference: 0.0,
            bpm: 0.0,
            previous_bpm_ratings: vec![],
            bpm_target: 120,
            epsilon: 80,
            prev_bpm_store_count: 10,
            displaying_indicator: true
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { .. } = self;
        let now = wasm_timer::SystemTime::now();
        let delta = now.duration_since(self.time).unwrap_or_default().as_secs_f64();

        ctx.request_repaint();

        #[cfg(debug_assertions)]
        let mousepos = match ctx.pointer_hover_pos() {
            None => {Pos2::new(0.0,0.0)}
            Some(a) => {a}
        };

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading(format!("Frame time: {:.05} seconds", delta));

            #[cfg(debug_assertions)]
            ui.label(format!("{:?}", self.time));

            let prev_button_click = self.button_click_time;

            let big_button = ui.add_sized([100.0,50.0],egui::Button::new("Click"));

            if big_button.clicked() {

                self.button_click_time = wasm_timer::SystemTime::now();

                let difference = {
                    self.button_click_time.duration_since(prev_button_click).unwrap_or_default().as_nanos() as f64 / 1_000_000 as f64
                };

                println!("difference: {}", difference);
                self.prev_difference = difference;
                self.bpm = 60_000.0 / difference;
                println!("{}", self.bpm);
                self.previous_bpm_ratings.push(self.bpm as f32);
            }

            ui.horizontal(|ui| {
                ui.label("Leeway:");
                ui.add_space(24.25);
                ui.add(egui::Slider::new(
                    &mut self.epsilon,
                    0..=200)).on_hover_text(
                    "The length of time to show the green indicator and click text."
                );
            });

            ui.horizontal(|ui| {
                ui.label("BPM Target:");
                ui.add(egui::Slider::new(
                    &mut self.bpm_target,
                    100..=300)).on_hover_text(
                    "The bpm to show the indicator at."
                );
            });

            ui.horizontal(|ui| {
                ui.label("Avg Count:");
                ui.add_space(8.0);
                ui.add(
                    egui::Slider::new(
                        &mut self.prev_bpm_store_count,
                        2..=50)).on_hover_text(
                    "The number of bpm ratings to keep to calculate the average."
                );
            });

            ui.checkbox(&mut self.displaying_indicator,"Display indicator: ");

            let difference_check:i128 = {
                let output = now.duration_since(prev_button_click).unwrap().as_millis() as i128;
                if output > 10_000 {
                    10_000
                } else { output }
            };

            let bpm_ms = 60_000 / self.bpm_target; // 500 ms?

            let click_offset = 300.0;

            let x: f32 = {
                bpm_ms - difference_check as i32 + click_offset as i32
            } as f32;

            // rectangle object for the indicator bar, moves from right to left of screen towards the click rect.
            let indicator_rect = Rect::from_two_pos(
                Pos2::new(x,200.0),
                Pos2::new(x + 10.0,300.0)
            );

            // rectangle to show where the indicator needs to be to click the button
            let click_rect = Rect::from_two_pos(
                Pos2::new(click_offset, 200.0),
                Pos2::new(click_offset + 5.0, 300.0)
            );



            if self.displaying_indicator {

                ui.painter().rect(
                    click_rect,
                    Rounding::default(),
                    Color32::from_rgb(50,50,50),
                    Stroke::default()
                );

                ui.painter().rect(
                    indicator_rect,
                    Rounding::default(),
                    Color32::from_rgb(250,50,50),
                    Stroke::default()
                );

            }

            if (difference_check - (bpm_ms - (self.epsilon/2)) as i128).abs() < self.epsilon as i128 && self.displaying_indicator {

                let rect = Rect::from_two_pos(
                    Pos2::new(click_offset,200.0),
                    Pos2::new(click_offset + 5.0,300.0)
                );

                ui.painter().rect(
                    rect,
                    Rounding::default(),
                    Color32::from_rgb(50,200,50),
                    Stroke::default()
                );
                ui.painter().text(
                    Pos2::new(click_offset, 180.0),
                    Align2::CENTER_BOTTOM,
                    "Click now!",
                    FontId::default(),
                    Color32::from_rgb(250,250,250)
                );

            }

            ui.label(format!("Previous BPM: {:.0}",self.bpm));

            let mut avg: f32 = 0.0;

            for rating in &self.previous_bpm_ratings {
                avg += rating;
            }

            avg /= self.previous_bpm_ratings.len() as f32;

            if !avg.is_nan() {
                ui.label(format!("BPM Avg: {:.0}",avg));
            }
            // 60,000 / 120 = 500 ms per beat
            // 1 ms = 1,000,000 nanos

            if ui.button("Reset Average").clicked() {
                self.previous_bpm_ratings.clear();
            }

            if self.previous_bpm_ratings.len() > self.prev_bpm_store_count {
                self.previous_bpm_ratings.remove(0);
            }


            #[cfg(debug_assertions)]
            ui.label(format!("{:?}",mousepos));
            #[cfg(debug_assertions)]
            ui.label(format!("{}, {}", x, difference_check));

            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("test").show(ctx, |ui| {
                ui.label("This is a test window.");

            });
        }

        if self.frames > 60 {
            self.frames = 0;
        }

        self.frames = self.frames + 1;

        self.time = wasm_timer::SystemTime::now();
    }
}
