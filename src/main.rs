use std::io::{Error as IOError, stdin, stdout, Write};
use std::num::ParseFloatError;
use std::process::exit;
use std::str::FromStr;

const MAX_ITERATIONS_COUNT: i64 = 1000000000;
const DEFAULT_EPSILON: f64 = 0.0001;
const EXIT_INCORRECT_LOWER_BOUND: i32 = 1;
const EXIT_INCORRECT_UPPER_BOUND: i32 = 2;

fn parse_to_double(str: &str) -> Result<f64, ParseFloatError> {
    Ok(f64::from_str(str)?)
}

pub fn get_line<'a>() -> Result<&'a str, IOError> {
    let mut result: String = String::new();
    stdin().read_line(&mut result)?;
    result = String::from(result.trim());
    let result: &'a str = result.leak::<'a>();
    Ok(result)
}

// FIXME function calculates not all roots. 
// Make better algorithm than simply iterating over a domain in which we are finding roots.
fn find_roots<'a>(
    function: fn(f64) -> f64,
    derivative: fn(f64) -> f64,
    second_derivative: fn(f64) -> f64,
    epsilon: f64,
    lower_bound: f64,
    upper_bound: f64,
) -> &'a [f64] {
    let mut list: Vec<f64> = Vec::new();
    let mut current_iteration_lower_bound = lower_bound;
    let iteration_step = epsilon * 10.0;
    while current_iteration_lower_bound + iteration_step <= upper_bound {
        match find_root_newton(
            function, derivative, second_derivative,
            current_iteration_lower_bound, epsilon, MAX_ITERATIONS_COUNT,
        ) {
            None => {
                current_iteration_lower_bound += iteration_step;
                continue;
            }
            Some(root) => {
                if root < lower_bound || root > upper_bound {
                    current_iteration_lower_bound += iteration_step;
                    continue;
                }
                if list.iter().any(|x| { (root - x).abs() < epsilon }) {
                    current_iteration_lower_bound += iteration_step;
                    continue;
                }
                list.push(root);
                current_iteration_lower_bound += iteration_step;
            }
        };
    }
    list.leak::<'a>()
}

fn find_root_newton(
    function: fn(f64) -> f64,
    derivative: fn(f64) -> f64,
    second_derivative: fn(f64) -> f64,
    x0: f64,
    epsilon: f64,
    max_iterations: i64,
) -> Option<f64> {
    if function(x0) * second_derivative(x0) <= 0f64 {
        return None;
    }
    let mut x = x0;
    let mut i = 0i64;
    while i < max_iterations {
        let fx = function(x);
        let dfx = derivative(x);
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

// TODO make user enter function in standard input or a file
fn function(x: f64) -> f64 {
    x * x * x * x - 4.3 * x * x * x - 1.29 * x * x + 15.1 * x.sin() + 9.84
}

// TODO calculate first and second derivative numerically rather than explicitly specifying in code.
fn derivative(x: f64) -> f64 {
    (151.0 / 10.0) * x.cos() + 4.0 * x * x * x - 12.9 * x * x - (129.0 / 50.0) * x
}

fn second_derivative(x: f64) -> f64 {
    (1.0 / 50.0) * (-755.0 * x.sin() + 600.0 * x * x - 1290.0 * x - 129.0)
}

fn main() {
    print!("Введите нижнюю границу: ");
    stdout().flush().unwrap();
    let lower_bound = parse_to_double(get_line().unwrap())
        .inspect_err(|_| {
            eprintln!("Ошибка преобразования ввода в вещественное число");
            exit(EXIT_INCORRECT_LOWER_BOUND);
        })
        .unwrap();
    print!("Введите верхнюю границу: ");
    stdout().flush().unwrap();
    let upper_bound = parse_to_double(get_line().unwrap())
        .inspect_err(|_| {
            eprintln!("Ошибка преобразования ввода в вещественное число");
            exit(EXIT_INCORRECT_UPPER_BOUND);
        })
        .unwrap();
    let roots =
        find_roots(
            function,
            derivative,
            second_derivative,
            DEFAULT_EPSILON,
            lower_bound,
            upper_bound);
    println!("{:?}", roots)
}
