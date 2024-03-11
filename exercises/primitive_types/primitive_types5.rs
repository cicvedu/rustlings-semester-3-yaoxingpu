/*
 * @Author: yaoxingpu yaoxpu@163.com
 * @Date: 2024-03-11 11:52:39
 * @LastEditors: yaoxingpu yaoxpu@163.com
 * @LastEditTime: 2024-03-11 17:11:50
 * @FilePath: /rustlings-semester-3-yaoxingpu/exercises/primitive_types/primitive_types5.rs
 * @Description:
 *
 */
// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name,age) /* your pattern here */ = cat;

    println!("{} is {} years old.", name, age);
}
