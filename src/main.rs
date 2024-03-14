extern crate core;

const MAX_ITERATIONS_COUNT: i64 = 1000000000;

fn find_roots(
    f: fn(f64) -> f64,
    df: fn(f64) -> f64,
    df2: fn(f64) -> f64,
    epsilon: f64,
    lower_bound: f64,
    upper_bound: f64,
) -> Vec<f64> {
    let mut list: Vec<f64> = Vec::new();
    let mut i = lower_bound;
    let step = epsilon * 10.0;
    while i + step <= upper_bound {
        match find_root_newton(f, df, df2, i, epsilon, MAX_ITERATIONS_COUNT) {
            None => {
                i += step;
                continue;
            }
            Some(root) => {
                if root < lower_bound || root > upper_bound {
                    i += step;
                    continue;
                }
                if list.iter().any(|x| { (root - x).abs() < epsilon }) {
                    i += step;
                    continue;
                }
                list.push(root);
                i += step;
            }
        };
    }
    list
}

fn find_root_newton(
    f: fn(f64) -> f64,
    df: fn(f64) -> f64,
    df2: fn(f64) -> f64,
    x0: f64,
    epsilon: f64,
    max_iterations: i64,
) -> Option<f64> {
    if f(x0) * df2(x0) <= 0f64 {
        return None;
    }
    let mut x = x0;
    let mut i = 0i64;
    while i < max_iterations {
        let fx = f(x);
        let dfx = df(x);
        if fx.abs() < epsilon {
            return Some(x);
        }
        x -= fx / dfx;
        i += 1;
    }
    if i == max_iterations {
        return None;
    }
    Some(x)
}

fn main() {
    let roots =
        find_roots(
            |x| {
                x * x * x * x - 4.3 * x * x * x - 1.29 * x * x + 15.1 * x.sin() + 9.84
            },
            |x| {
                (151.0 / 10.0) * x.cos() + 4.0 * x * x * x - 12.9 * x * x - (129.0 / 50.0) * x
            },
            |x| {
                (1.0 / 50.0) * (-755.0 * x.sin() + 600.0 * x * x - 1290.0 * x - 129.0)
            },
            0.0001,
            0.0,
            4.0);
    println!("{:?}", roots)
}
