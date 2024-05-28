fn main() {
    let num1 = 8;
    let num2 = 4;

    if num1 > num2 {
        println!("{} is greater than {}", num1, num2);
    } else if num1 < num2 {
        println!("{} is less than {}", num1, num2);
    } else {
        println!("{} is equal to {}", num1, num2);
    }

    println!("fact(5): {}", fact(5));
}

fn func1() {
    println!("func1");
}

fn fact(n: u64) -> u64 {
    if n == 0 {
        return 1;
    } else {
        return n * fact(n - 1);
    }
}