/*
 * @Author: yaoxingpu yaoxpu@163.com
 * @Date: 2024-03-11 11:52:39
 * @LastEditors: yaoxingpu yaoxpu@163.com
 * @LastEditTime: 2024-03-11 16:43:51
 * @FilePath: /rustlings-semester-3-yaoxingpu/exercises/functions/functions3.rs
 * @Description:
 *
 */
// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.



fn main() {
    call_me(5);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
