use eframe::epaint::{Color32, Pos2, Rect, Rounding, Stroke};

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

    bpm_target: i32,
    epsilon: i32,

}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            time: wasm_timer::SystemTime::now(),
            frames: 0,
            button_click_time: wasm_timer::SystemTime::now(),
            prev_difference: 0.0,
            bpm: 0.0,
            bpm_target: 120,
            epsilon: 80,
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

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.label("Hello noah and will");
            // ui.label("test");
            ui.heading(format!("{}", delta));


            ui.label(format!("{:?}", self.time));

            let prev_button_click = self.button_click_time;

            if ui.button("click").clicked() {

                self.button_click_time = wasm_timer::SystemTime::now();

                let difference = {
                    self.button_click_time.duration_since(prev_button_click).unwrap_or_default().as_nanos() as f64 / 1_000_000 as f64
                };
                println!("difference: {}", difference);
                self.prev_difference = difference;
                self.bpm = 60_000.0 / difference;
                println!("{}", self.bpm);
            }

            ui.label("Leeway: ");
            ui.add(egui::Slider::new(&mut self.epsilon,0..=200));
            ui.label("BPM Target: ");
            ui.add(egui::Slider::new(&mut self.bpm_target,100..=300));


            let difference_check:i128 = now.duration_since(prev_button_click).unwrap().as_millis() as i128;
            let bpm_ms = 60_000 / self.bpm_target; // 500 ms?

            if (difference_check - (bpm_ms - (self.epsilon/2)) as i128).abs() < self.epsilon as i128 {
                let rect = Rect::from_two_pos(Pos2::new(200.0,200.0),Pos2::new(300.0,300.0));
                ui.painter().rect(rect,Rounding::default(),Color32::from_rgb(250,50,50),Stroke::default());
            }

            ui.label(format!("BPM: {:.0}",self.bpm));
            // 60,000 / 120 = 500 ms per beat
            // 1 ms = 1,000,000 nanos

            //

            egui::warn_if_debug_build(ui);
        });

        if true {
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
