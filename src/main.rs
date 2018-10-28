fn main() {
    println!("Hello, world!");
    print!("{}",fact(5));
}

fn fact( n: i32) -> i32 {
    return if n==0{
        1
    }else{
        n * fact(n -1)
    }
}