const STEP_SIZE: f64 = 1e-6;

fn main() {
    // solve int_x^2 sin(x) = 0.5
    let f = |x: f64| x.sin();
    let x = solve_integral_lower_bound(&f, 2., 0.5, STEP_SIZE);

    println!("int^2_{} sin x dx = 0.5", x);
}



/**
 * Solve equation `integral = \int^{upper_bound}_x f(t) dt` for `x`.
 */
fn solve_integral_lower_bound(
    f: &dyn Fn(f64) -> f64,
    upper_bound: f64,
    integral: f64,
    step_size: f64
) -> f64
{
    let mut sum = 0.;
    let mut x = upper_bound;
    let mut last_value = f(x - step_size);
    let sign = integral > 0.;

    loop {
        let curr_value = f(x);
        sum += trapezoid_area(step_size, last_value, curr_value);
        last_value = curr_value;
        x -= step_size;

        if (integral > sum) != sign { return x + 0.5 * step_size; }
    }
}

/**
 * Compute the area of a right trapezoid of base `dx` and heights `y1` and `y2`
 */
fn trapezoid_area(dx: f64, y1: f64, y2: f64) -> f64 {
    return 0.5 * dx * (y1 + y2);
}
