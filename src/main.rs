// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

pub fn fizz_buzz(num: i32) -> String {
    if num % 15 == 0 {
        return "fizzbuzz".to_string();
    } else if num % 3 == 0 {
        return "fizz".to_string();
    } else if num % 5 == 0 {
        return "buzz".to_string();
    } else {
        return num.to_string();
    }
}

fn main() {

    assert_eq!(fizz_buzz(15),"fizzbuzz".to_string());
    assert_eq!(fizz_buzz(3),"fizz".to_string());
    assert_eq!(fizz_buzz(5),"buzz".to_string());
    assert_eq!(fizz_buzz(13),"13".to_string());

    // println!("Guess the number!");
    // let secret_number = rand::thread_rng().gen_range(1..101).to_string();
    // println!("The secret number is:{}", secret_number);
    // println!("Please input your guess.");
    // let mut guess = String::new();
    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");
    // println!("You guessed: {}", guess);

    // match guess.cmp(&secret_number) {
    //     Ordering::Less => println!("Too small!"),
    //     Ordering::Greater => println!("Too big!"),
    //     Ordering::Equal => println!("You win!"),
    // }
}
