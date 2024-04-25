fn create_sin(start : f64, stop : f64, steps : u32) -> Vec<f64> {
    let mut list : Vec<_>  = Vec::new();
    let step_size : f64 = (stop - start)/(steps as f64);
    for i in 0..steps {
        list.push(start + (i as f64)*step_size);
    }
    let list : Vec<_> = list.iter()
                        .map(|&x| (x as f64)*2.0*std::f64::consts::PI)
                        .map(|x| x.sin())
                        .collect();
    list
}


fn main() {
    let list = create_sin(0.0, 2.0, 30);
    println!("Hello, world!");
    println!("{:?}", list);
}
