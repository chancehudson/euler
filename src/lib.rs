/// a 50 bit euler approximation in f64
/// returns e^x
pub fn euler_50_approx(x: f64) -> f64 {
    let p1: f64 = 1.66666666666666019037 * 10_f64.powf(-1_f64);
    let p2: f64 = -2.77777777770155933842 * 10_f64.powf(-3_f64);
    let p3: f64 = 6.61375632143793436117 * 10_f64.powf(-5_f64);
    let p4: f64 = -1.65339022054652515390_f64 * 10_f64.powf(-6_f64);
    let p5 = 4.13813679705723846039 * 10_f64.powf(-8_f64);
    let s = x / 2_f64;
    let t = s * s;
    // Let c = s−t·(p1 +t·(p2 +t·(p3 +t·(p4 +t·p5))))
    // explicitly write horner evaluation i guess
    let c = s - t * (p1 + t * (p2 + t * (p3 + t * (p4 + t * p5))));
    // Let r = 1 − ((s · c) / (c − 2) − s).
    let r = 1_f64 - ((s * c) / (c - 2_f64) - s);
    return r * r;
}

/// a 50 bit euler approximation in f64
/// returns e^x
///
/// alias for euler_50_approx
pub fn exp(x: f64) -> f64 {
    euler_50_approx(x)
}

#[test]
pub fn calculate_euler() {
    // produce a warning if the value passed to `exp` is > 400?
    // values above x = 400 converge on 1
    for z in 0..1000_u32 {
        println!("{z}");
        println!("{}", euler_50_approx(z as f64));
    }
}
