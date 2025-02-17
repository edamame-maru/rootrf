use std::env;

fn main() {
    // easier to deal with cmdline arguments
    let args: Vec<String> = env::args().collect();
    let mut coeffs: Vec<f32> = vec![];

    // coeffs must be f32 to deal with adding h
    for i in 1..args.len() {
        let coeff: f32 = args[i].trim().parse().unwrap();
        coeffs.push(coeff);
    }

    println!("{:?}", coeffs);

    // find roots
    let mut x: f32 = 0.0;
    for _ in 0..100000 {
        let mut current: f32 = 0.0;
        let mut forward: f32 = 0.0;
        for i in 0..coeffs.len() {
            current += coeffs[i] * x.powf((coeffs.len() - (i + 1)) as f32);
            forward += coeffs[i] * (x + 0.000001).powf((coeffs.len() - (i + 1)) as f32);
        }
        let m = (forward - current) / 0.000001;


        let c = current - m * x;
        x = -(c / m);
    }

    println!("{}", x);
}
