use std::io;
use std::io::Write;

const ITER: i32 = 100;
const STEP: f32 = 0.000001;
const STRT: f32 = 10.0;

fn f(x: f32) -> f32 {
    (x - 1.0) * (x - 2.0) * (x - 3.0)
}
fn findr(u: f32) -> f32 {
    let mut x = u;
    for _ in 0..ITER {
        let m = (f(x + STEP) - f(x)) / STEP;
        let c = f(x) - m * x;
        x = -(c / m);
    }

    x
}

fn main() {
    print!("degree: ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read line");

    let degree: i32 = buffer.trim().parse().unwrap();

    // user needs to see the polynomial with no coefficients
    for i in 0..=degree {
        if i == 0 {
            print!("f(x) = ");
        }

        if i == degree {
            println!("_");
        } else {
            print!("_x^{} + ", degree - i);
        }
    }

    let mut coeffs: Vec<i32> = vec![];

    for i in 0..=degree {
        if i == degree {
            print!("enter _ for constant term <_>: ");
        } else {
            print!("enter _ for term <_>x^{}: ", degree - i);
        }
        io::stdout().flush().unwrap();

        buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read line");

        let coeff: i32 = buffer.trim().parse().unwrap();
        coeffs.push(coeff);
    }

    println!("{:?}", coeffs)

    //println!("f(x) = (x - 1)(x - 2)(x - 3):\n => f({}) = 0", findr(STRT));
}
