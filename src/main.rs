fn main() {
    let max: i16 = 100;
    fizz_buzz(max);
}

fn fizz_buzz(range: i16) -> () {
    for x in 1..range+1 {
        let three_div = if x % 3 == 0 { true } else { false };
        let five_div  = if x % 5 == 0 { true } else { false };

        if three_div && five_div {
            println!("fizzbuzz");
        }
        else if three_div {
            println!("fizz");
        }
        else if five_div {
            println!("buzz");
        }
        else {
            println!("{}", x);
        }
    }
}