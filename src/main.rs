use eframe::egui::{self, DragValue, Slider, Event, Vec2};
use egui_plot::{Line, Legend, PlotPoints};

mod rdp;

enum MathFunc {
    Sin,
    Exp,
}

pub struct PlotExample {
    lock_x: bool,
    lock_y: bool,
    ctrl_to_zoom: bool,
    shift_to_horizontal: bool,
    zoom_speed: f32,
    scroll_speed: f32,
    range_start: f64,
    range_end: f64,
    range_steps: u32,
    func: MathFunc,
    epsilon: f64,
}

impl Default for PlotExample {
    fn default() -> Self {
        Self {
            lock_x: false,
            lock_y: false,
            ctrl_to_zoom: false,
            shift_to_horizontal: false,
            zoom_speed: 1.0,
            scroll_speed: 1.0,
            range_start: 0.0,
            range_end: 1.0,
            range_steps: 300,
            func: MathFunc::Sin,
            epsilon: 0.05,
        }
    }
}

impl eframe::App for PlotExample {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::TopBottomPanel::top("options").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        Slider::new(&mut self.range_start, 0.0..=5.0)
                    );
                    ui.label("start value").on_hover_text("TBD");
                });
                ui.horizontal(|ui| {
                    ui.add(
                        Slider::new(&mut self.range_end, 0.0..=5.0)
                    );
                    ui.label("end value").on_hover_text("TBD");
                });
            });
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        Slider::new(&mut self.range_steps, 10..=1000)
                    );
                    ui.label("steps").on_hover_text("TBD");
                });
                ui.horizontal(|ui| {
                    ui.add(
                        Slider::new(&mut self.epsilon, 0.0..=0.5)
                    );
                    ui.label("epsilon").on_hover_text("TBD");
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let (scroll, pointer_down, modifiers) = ui.input(|i| {
                let scroll = i.events.iter().find_map(|e| match e {
                    Event::MouseWheel {
                        unit: _,
                        delta,
                        modifiers: _,
                    } => Some(*delta),
                    _ => None,
                });
                (scroll, i.pointer.primary_down(), i.modifiers)
            });

            egui_plot::Plot::new("plot")
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .legend(Legend::default())
                .show(ui, |plot_ui| {
                    if let Some(mut scroll) = scroll {
                        if modifiers.ctrl == self.ctrl_to_zoom {
                            scroll = Vec2::splat(scroll.x + scroll.y);
                            let mut zoom_factor = Vec2::from([
                                (scroll.x * self.zoom_speed / 10.0).exp(),
                                (scroll.y * self.zoom_speed / 10.0).exp(),
                            ]);
                            if self.lock_x {
                                zoom_factor.x = 1.0;
                            }
                            if self.lock_y {
                                zoom_factor.y = 1.0;
                            }
                            plot_ui.zoom_bounds_around_hovered(zoom_factor);
                        } else {
                            if modifiers.shift == self.shift_to_horizontal {
                                scroll = Vec2::new(scroll.y, scroll.x);
                            }
                            if self.lock_x {
                                scroll.x = 0.0;
                            }
                            if self.lock_y {
                                scroll.y = 0.0;
                            }
                            let delta_pos = self.scroll_speed * scroll;
                            plot_ui.translate_bounds(delta_pos);
                        }
                    }
                    if plot_ui.response().hovered() && pointer_down {
                        let mut pointer_translate = -plot_ui.pointer_coordinate_drag_delta();
                        if self.lock_x {
                            pointer_translate.x = 0.0;
                        }
                        if self.lock_y {
                            pointer_translate.y = 0.0;
                        }
                        plot_ui.translate_bounds(pointer_translate);
                    }
                    let mut some_points : Vec<rdp::Point> = rdp::create_sin(&self.range_start, &self.range_end, &self.range_steps);
                    let plot_points : Vec<[f64;2]> = some_points.clone().iter().map(|x| x.as_arr()).collect();
                    let sine_points = PlotPoints::from(plot_points);
                    plot_ui.line(Line::new(sine_points).name("somepoints"));
                    // let mut some_points : Vec<Point> = create_sin(0.0, 2.0, 300);
                    let some_points : Vec<[f64;2]> = rdp::rdp_alg(&mut some_points, &self.epsilon).unwrap().iter().map(|x| x.as_arr()).collect();
                    let sine_points = PlotPoints::from(some_points);
                    plot_ui.line(Line::new(sine_points).name("rdp"));
                });
        });
    }
}


fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Plot",
        options,
        Box::new(|_cc| Box::<PlotExample>::default()),
    )
}
