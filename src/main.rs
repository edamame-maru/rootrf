const ITER: i32 = 100;
const STEP: f32 = 0.000001;
const STRT: f32 = 10.0;

// f(x) = (x - 1)(x - 2)(x - 3)
fn f(x: f32) -> f32 {
    (x - 1.0) * (x - 2.0) * (x - 3.0)
}

fn findr(u: f32) -> f32 {
    let mut x: f32 = u;
    for _ in 0..ITER {
        let m = (f(x + STEP) - f(x)) / STEP;
        let c = f(x) - m * x;
        x = - (c / m);
    }

    x
}

fn main() {
    println!("f(x) = (x - 1)(x - 2)(x - 3):\n => f({}) = 0", findr(STRT));
}
