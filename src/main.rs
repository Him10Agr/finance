fn present_value(future_value: f64, rate: f64, periods: u32) -> f64 {

    future_value / (1.0 + rate).powi(periods as i32) 
}

fn main() {
    let fv = 1000.0;
    let rate = 0.05;
    let periods = 10;
    println!("Present Value: {}", present_value(fv, rate, periods));
}