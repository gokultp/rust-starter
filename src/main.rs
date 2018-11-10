fn main() {
    println!("Hello, world!");
    println!("{}",fact(5));
    println!("{}", test_match(true));
    test_types();
}

fn fact( n: i32) -> i32 {
    return if n==0{
        1
    }else{
        n * fact(n -1)
    }
}
fn test_match(boolean: bool) ->i32 {
    let val = match boolean {
       true =>1,
       false =>0,
    };
    return val;
}

fn test_types(){
    // int
    let a = 5;
    let b :i32 = 6;
    println!("{} + {} = {}",a, b, a+b);

    // float
    let fl1 = 5.2;
    let fl2 :f64 = 6.8;
    println!("{} + {} = {}",fl1, fl2, fl1+fl2);

    // tuple
    t :(i32, f32, char) = (1,2.0, 'a');

}