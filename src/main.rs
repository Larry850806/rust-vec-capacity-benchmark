use std::time::Instant;

fn double(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec2 = Vec::new();
    for x in vec.iter() {
        vec2.push(x * 2);
    }
    return vec2;
}

fn double2(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec2 = Vec::with_capacity(vec.len());
    for x in vec.iter() {
        vec2.push(x * 2);
    }
    return vec2;
}

fn main() {
    let mut vec = Vec::new();

    for x in 0..100000 {
        vec.push(x);
    }

    let mut total_time: u128 = 0;
    {
        for _ in 0..1000 {
            let now = Instant::now();
            double(&vec);
            total_time += now.elapsed().as_micros();
        }
        println!("double:  {}μs for 1000 times", total_time);
    }

    let mut total_time2: u128 = 0;
    {
        for _ in 0..1000 {
            let now = Instant::now();
            double2(&vec);
            total_time2 += now.elapsed().as_micros();
        }
        println!("double2: {}μs for 1000 times", total_time2);
    }

    println!(
        "double2 is {:.2} times faster than double",
        total_time as f64 / total_time2 as f64
    );
}
