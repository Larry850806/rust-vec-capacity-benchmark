#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate test;

fn double(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec2 = Vec::new();
    for x in vec.iter() {
        vec2.push(x * 2);
    }
    return vec2;
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7];

    let vec2 = double(&vec);

    println!("{:?}", vec);
    println!("{:?}", vec2);
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }
}
