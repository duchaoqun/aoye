// main 函数，代表程序的入口，函数是通过 fn 关键字定义的。
// 函数，传入 i32类型参数，返回 String 类型
// 如果不指定返回类型，默认返回 () 单元类型。
// return 可以提前返回内容，如果不指定也返回 () 单元类型。

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
    // 使用 assert 来测试代码。
    assert_eq!(fizz_buzz(15),"fizzbuzz".to_string());
    assert_eq!(fizz_buzz(3),"fizz".to_string());
    assert_eq!(fizz_buzz(5),"buzz".to_string());
    assert_eq!(fizz_buzz(13),"13".to_string());
}
