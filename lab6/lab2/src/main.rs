fn check_even(num: i32) -> bool {
    num % 2 == 0
}

fn main() {
    let numbers = [10, 15, 20, 25, 30];
    for num in numbers {
        if check_even(num) {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }
    }
}
