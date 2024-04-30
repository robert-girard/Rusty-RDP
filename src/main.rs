use eframe::egui::{self, DragValue, Event, Vec2};
use egui_plot::{Line, Legend, PlotPoints, PlotPoint};
use std::f64::consts::PI;
use std::env;

fn create_sin(start : f64, stop : f64, steps : u32) -> Vec<Point> {
    let mut list : Vec<_>  = Vec::new();
    let step_size : f64 = (stop - start)/((steps -1) as f64);
    for i in 0..(steps) {
        list.push(start + (i as f64)*step_size);
    }
    let list : Vec<_> = list.iter()
                        .map(|&x| Point {y: ((x as f64)*2.0*PI).sin(), x})
                        .collect();
    list
}

#[derive(Debug, Clone, Copy)]
struct Point {
    y : f64,
    x : f64
}

impl Point {
    fn as_plot_point(&self) -> PlotPoint {
        return PlotPoint::new(self.x, self.y);
    }
    fn as_arr(&self) -> [f64;2] {
        return [(&self.x).to_owned(),(&self.y).to_owned()];
    }
}

struct Range {
    start : f64,
    end : f64,
    steps : u32,
    current : f64,
}

impl Range {
    fn new(start : f64, end : f64, steps : u32) -> Range {
        Range {start:start, end:end, steps:steps, current:start - (end-start)/(steps as f64)}
    }
}

impl Iterator for Range {
    type Item = f64;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.current += (self.end - self.start)/(self.steps as f64);
        if self.current >= self.end{
            None
        } else {
            Some(self.current)
        }
    }
}


// struct regularIntervalpoint

fn perpendicular_distance(l1 : &Point, l2 : &Point, p : &Point) -> f64{
    let num = ((l2.x-l1.x)*(p.y-l1.y) - (p.x - l1.x)*(l2.y-l1.y)).abs();
    let den = ((l2.x-l1.x).powi(2) + (l2.y-l1.y).powi(2)).sqrt();
    num/den
}

fn vertical_distance(l1 : &Point, l2 : &Point, p : &Point) -> f64{
    let slope = (l2.y-l1.y)/(l2.x-l1.x);
    let x_dist = p.x-l1.x;
    let line_y = l1.y + x_dist*slope;
    (line_y - p.y).abs()
}

fn rdp_alg(data : &mut Vec<Point>, eps : f64) -> Option<Vec<Point>> {
    let mut dmax : f64 = 0.0;
    let mut imax =  0;
    let l1 = data.first()?;
    let l2 = data.last()?;
    for i in 1..(data.len()-2) {
        let p = data.get(i)?;
        let d = vertical_distance(l1, l2, p);
        if d > dmax {
            imax = i;
            dmax = d;
        }
    }
    if dmax > eps {
        let mut first_half = data.clone();
        let mut second_half = first_half.split_off(imax);
        first_half.push(second_half.first().unwrap().clone());
        let mut first = rdp_alg(&mut first_half, eps)?;
        let mut second = rdp_alg(&mut second_half, eps)?;
        first.pop();
        first.append(&mut second);
        return Some(first);
    }
    let data = vec![data.first()?.to_owned(), data.last()?.to_owned()];
    return Some(data);
}


fn main() -> Result<(), eframe::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    let list : Vec<PlotPoint> = create_sin(0.0, 2.0, 30).iter().map(|x| x.as_plot_point()).collect();
    // println!("Hello, world!");
    // println!("{:?}", list);
    // let range = Range::new(0.0,10.0,10);
    // let vals : Vec<_> = range.collect();
    // println!("{:?}", vals);
    // let mut list : Vec<Point> = create_sin(0.0, 2.0, 30);
    // println!("{:?}", rdp_alg(&mut list, 0.2));


    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Plot",
        options,
        Box::new(|_cc| Box::<PlotExample>::default()),
    )
}

struct PlotExample {
    lock_x: bool,
    lock_y: bool,
    ctrl_to_zoom: bool,
    shift_to_horizontal: bool,
    zoom_speed: f32,
    scroll_speed: f32,
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
        }
    }
}

impl eframe::App for PlotExample {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::SidePanel::left("options").show(ctx, |ui| {
            ui.checkbox(&mut self.lock_x, "Lock x axis").on_hover_text("Check to keep the X axis fixed, i.e., pan and zoom will only affect the Y axis");
            ui.checkbox(&mut self.lock_y, "Lock y axis").on_hover_text("Check to keep the Y axis fixed, i.e., pan and zoom will only affect the X axis");
            ui.checkbox(&mut self.ctrl_to_zoom, "Ctrl to zoom").on_hover_text("If unchecked, the behavior of the Ctrl key is inverted compared to the default controls\ni.e., scrolling the mouse without pressing any keys zooms the plot");
            ui.checkbox(&mut self.shift_to_horizontal, "Shift for horizontal scroll").on_hover_text("If unchecked, the behavior of the shift key is inverted compared to the default controls\ni.e., hold to scroll vertically, release to scroll horizontally");
            ui.horizontal(|ui| {
                ui.add(
                    DragValue::new(&mut self.zoom_speed)
                        .clamp_range(0.1..=2.0)
                        .speed(0.1),
                );
                ui.label("Zoom speed").on_hover_text("How fast to zoom in and out with the mouse wheel");
            });
            ui.horizontal(|ui| {
                ui.add(
                    DragValue::new(&mut self.scroll_speed)
                        .clamp_range(0.1..=100.0)
                        .speed(0.1),
                );
                ui.label("Scroll speed").on_hover_text("How fast to pan with the mouse wheel");
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

            ui.label("This example shows how to use raw input events to implement different plot controls than the ones egui provides by default, e.g., default to zooming instead of panning when the Ctrl key is not pressed, or controlling much it zooms with each mouse wheel step.");

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
                    let some_points : Vec<[f64;2]> = create_sin(0.0, 2.0, 300).iter().map(|x| x.as_arr()).collect();
                    let sine_points = PlotPoints::from(some_points);
                    plot_ui.line(Line::new(sine_points).name("somepoints"));
                    let mut some_points : Vec<Point> = create_sin(0.0, 2.0, 300);
                    let some_points : Vec<[f64;2]> = rdp_alg(&mut some_points, 0.05).unwrap().iter().map(|x| x.as_arr()).collect();
                    let sine_points = PlotPoints::from(some_points);
                    plot_ui.line(Line::new(sine_points).name("rdp"));
                });
        });
    }
}