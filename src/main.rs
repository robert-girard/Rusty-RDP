fn create_sin(start : f64, stop : f64, steps : u32) -> Vec<Point> {
    let mut list : Vec<_>  = Vec::new();
    let step_size : f64 = (stop - start)/(steps as f64);
    for i in 0..steps {
        list.push(start + (i as f64)*step_size);
    }
    let list : Vec<_> = list.iter()
                        .map(|&x| Point {y: ((x as f64)*2.0*std::f64::consts::PI).sin(), x})
                        .collect();
    list
}

#[derive(Debug)]
struct Point {
    y : f64,
    x : f64
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

// fn rdp_alg(data : Vec<Point>, eps : f64) -> Vec<Point> {
//     let t = data.get(0).unwrap() + data.last().unwrap();
//     data;
// }


fn main() {
    let list = create_sin(0.0, 2.0, 30);
    println!("Hello, world!");
    println!("{:?}", list);
    let range = Range::new(0.0,10.0,10);
    let vals : Vec<_> = range.collect();
    println!("{:?}", vals);
}
