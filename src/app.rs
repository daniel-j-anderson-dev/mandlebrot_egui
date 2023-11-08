use mandlebrot::{
    Pixel,
    calculate_pixel_data,
    pixels_to_rgbimage,
    pixels_to_colorimage
};
use eframe::{
    App,
    CreationContext,
    egui::{
        Context,
        CentralPanel,
        DragValue, TextureOptions, TopBottomPanel, ScrollArea,
    },
    Frame,
    NativeOptions,
    AppCreator,
    HardwareAcceleration,
    epaint::TextureHandle,
};
use num::{
    Complex,
    Zero
};

pub const APP_NAME: &str = "Mandelbrot Set Image Generator";

pub fn native_options() -> NativeOptions {
    NativeOptions {
        resizable: true,
        active: true,
        vsync: true,
        hardware_acceleration: HardwareAcceleration::Preferred,
        centered: true,
        ..Default::default()
    }
}

pub fn app_creator() -> AppCreator {
    Box::new(|cc| { 
        Box::new(AppData::new(cc))
    })
}

#[derive(Default)]
pub struct AppData {
    image_width: usize,
    image_height: usize,
    scale_factor: f64,
    origin: Complex<f64>,
    iteration_max: usize,
    save_path: String,
    save_msg: String,
    generation_msg: String,
    texture: Option<TextureHandle>,
    pixel_data: Option<(Vec<Pixel>, std::time::Duration)>,
}
impl AppData {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        // TODO: Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            image_width: 640,
            image_height: 320,
            scale_factor: 1.0,
            origin: Complex::zero(),
            iteration_max: 1000,
            save_path: String::from("output/mandelbrot.png"),
            save_msg: String::from(""),
            generation_msg: String::from(""),
            pixel_data: None,
            texture: None,
        }
    }
    pub fn generate_pixel_data(&mut self) {
        let start = std::time::Instant::now();
        self.pixel_data = Some((
            calculate_pixel_data(
                self.image_width, self.image_height,
                self.scale_factor, self.origin,
                self.iteration_max
            ),
            std::time::Instant::now() - start
        ));
    }
}
impl App for AppData {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        // TODO: Make numeric widget
        // Top panel has settings
        TopBottomPanel::top("Inputs").show(ctx, |ui| {
            ui.heading("Mandelbrot Image Generator");
            ui.horizontal(|ui| {
                ui.label("Image width:");
                ui.add(DragValue::new(&mut self.image_width));
            });
            ui.horizontal(|ui| {
                ui.label("Image height:");
                ui.add(DragValue::new(&mut self.image_height));
            });
            ui.horizontal(|ui| {
                ui.label("Scale Factor:");
                ui.add(DragValue::new(&mut self.scale_factor));
            });
            ui.horizontal(|ui| {
                ui.label("Iteration Max:");
                ui.add(DragValue::new(&mut self.iteration_max));
            });
            ui.horizontal(|ui| {
                ui.label("Origin:");
                ui.label("real:");
                ui.add(DragValue::new(&mut self.origin.re));
                ui.label("imaginary");
                ui.add(DragValue::new(&mut self.origin.im));
            });
            ui.horizontal(|ui| {
                ui.label("Save path:");
                if ui.text_edit_singleline(&mut self.save_path).changed() {
                    self.save_msg = String::from("");
                }
            });
            ui.horizontal(|ui| {
                if ui.button("Generate").clicked() {
                    self.generate_pixel_data();
                    if let Some((ref pixels, time)) = self.pixel_data {
                        let output = pixels_to_colorimage(pixels, self.image_width, self.image_height);
                        self.texture = Some(ctx.load_texture("mandelbrot", output, TextureOptions::default()));
                        self.generation_msg = format!("Generated in {:?}", time);
                    }
                }
                if ui.button("Save").clicked() {
                    if let Some((ref pixels, _)) = self.pixel_data {
                        let output = pixels_to_rgbimage(pixels, self.image_width, self.image_height);
                        self.save_msg = match output.save(&self.save_path) {
                            Ok(()) => format!("Saved to {}", self.save_path),
                            Err(save_error) => save_error.to_string(),
                        };
                    } else {
                        self.save_msg = String::from("Nothing generated yet");
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label(&self.save_msg);
                ui.label(&self.generation_msg);
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            if let Some(ref texture) = self.texture {
                ScrollArea::new([true, true]).show(ui, |ui| {
                    ui.image(texture);
                });
            }
        });
    }
}