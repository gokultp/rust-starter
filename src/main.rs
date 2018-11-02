fn main() {
    println!("Hello, world!");
    print!("{}",fact(5));
    print!("{}", testMatch(17))
}

fn fact( n: i32) -> i32 {
    return if n==0{
        1
    }else{
        n * fact(n -1)
    }
}
fn testMatch(tshirt_size){
    let tshirt_size = match tshirt_width {
        16 => "S", // check 16
        17 | 18 => "M", // check 17 and 18
        19 ... 21 => "L", // check from 19 to 21 (19,20,21)
        22 => "XL",
        _ => "Not Available",
    };

}