/*
 * @Author: yaoxingpu yaoxpu@163.com
 * @Date: 2024-03-11 11:52:39
 * @LastEditors: yaoxingpu yaoxpu@163.com
 * @LastEditTime: 2024-03-12 15:13:13
 * @FilePath: /rustlings-semester-3-yaoxingpu/exercises/clippy/clippy2.rs
 * @Description:
 *
 */
// clippy2.rs
//
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let mut res = 42;
    let option = Some(12);
    while let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
