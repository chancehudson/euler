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
    use std::time::Instant;
    // produce a warning if the value passed to `exp` is > 400?
    // values above x = 400 converge on 1
    let mut total_ns: u128 = 0;
    let mut approx_vals = vec![];
    let mut native_vals = vec![];
    let iterations = 1000_usize;
    for z in 0..iterations {
        let now = Instant::now();
        let e = euler_50_approx(z as f64);
        approx_vals.push(e);
        // println!("{e}");
        let elapsed = now.elapsed();
        total_ns += elapsed.as_nanos();

        // println!("{z}");
        // println!("{z} {e}: {:.2?}", elapsed);
    }
    let average_ns = total_ns / u128::try_from(iterations).unwrap();
    println!("euler_50_approx avg: {average_ns} ns");

    total_ns = 0;

    for z in 0..iterations {
        let now = Instant::now();
        // let e = euler_50_approx(z as f64);
        let e = f64::exp(z as f64);
        native_vals.push(e);
        // println!("{e}");
        let elapsed = now.elapsed();
        total_ns += elapsed.as_nanos();

        // println!("{z}");
        // println!("{z} {e}: {:.2?}", elapsed);
    }
    for z in 0..iterations {
        // if native_vals[z] != approx_vals[z] {
        //     println!("mismatch at {z}");
        //     println!("{} != {}", native_vals[z], approx_vals[z]);
        // }
    }
    println!("rust euler avg: {average_ns} ns");
}
