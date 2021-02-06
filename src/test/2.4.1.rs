// main 函数，代表函数的程序的入口，函数是通过 fn 关键字定义的。
// 函数，传入 i32类型参数，返回 String 类型
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
·   
fn main() {
    assert_eq!(fizz_buzz(15),"fizzbuzz".to_string());
    assert_eq!(fizz_buzz(3),"fizz".to_string());
    assert_eq!(fizz_buzz(5),"buzz".to_string());
    assert_eq!(fizz_buzz(13),"13".to_string());
}
