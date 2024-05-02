use std::f64::consts::PI;

pub fn create_sin(start : &f64, stop : &f64, steps : &u32) -> Vec<Point> {
    let mut list : Vec<_>  = Vec::new();
    let step_size : f64 = (stop - start)/((steps -1) as f64);
    for i in 0..steps.clone() {
        list.push(start + (i as f64)*step_size);
    }
    let list : Vec<_> = list.iter()
                        .map(|&x| Point {y: ((x as f64)*2.0*PI).sin(), x})
                        .collect();
    list
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    y : f64,
    x : f64
}

impl Point {
    pub fn as_arr(&self) -> [f64;2] {
        return [(&self.x).to_owned(),(&self.y).to_owned()];
    }
    pub fn new(x : f64, y : f64) -> Point {
        Point {x : x, y : y}
    }
}

pub struct Range {
    start : f64,
    end : f64,
    steps : u32,
    current : f64,
}

impl Range {
    pub fn new(start : f64, end : f64, steps : u32) -> Range {
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

pub fn rdp_alg(data : &mut Vec<Point>, eps : &f64) -> Option<Vec<Point>> {
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
    if dmax > *eps {
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
