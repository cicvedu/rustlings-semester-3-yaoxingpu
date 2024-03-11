/*
 * @Author: yaoxingpu yaoxpu@163.com
 * @Date: 2024-03-11 11:52:39
 * @LastEditors: yaoxingpu yaoxpu@163.com
 * @LastEditTime: 2024-03-11 17:36:58
 * @FilePath: /rustlings-semester-3-yaoxingpu/exercises/move_semantics/move_semantics3.rs
 * @Description:
 *
 */
// move_semantics3.rs
//
// Make me compile without adding new lines-- just changing existing lines! (no
// lines with multiple semicolons necessary!)
//
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
