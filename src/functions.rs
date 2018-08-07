
pub fn test_fn() {
    println!("test!")
}

pub fn compose(f1: fn(i32) -> i32, f2: fn(i32) -> i32) -> impl Fn(i32) -> i32 {
    move |x| f1(f2(x))
}

//fn successor(n: i32) -> i32 { n + 1 }
//fn double(n:i32) -> i32 { n * 2 }
pub fn cube(n:i32) -> i32 { n * n * n }
pub fn square(n:i32) -> i32 { n * n }