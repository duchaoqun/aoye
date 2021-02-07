// if Expression
fn main() {
    let n = 13;
    // if Expression ，有一个返回值，的两个分支的结果类型一致，会去掉结尾的小数部分。
    let big_n = if n < 10 && n > -10 { 10 * n } else { n / 2 };

    assert_eq!(big_n, 6);
}
