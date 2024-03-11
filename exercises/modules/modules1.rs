/*
 * @Author: yaoxingpu yaoxpu@163.com
 * @Date: 2024-03-11 11:52:39
 * @LastEditors: yaoxingpu yaoxpu@163.com
 * @LastEditTime: 2024-03-11 20:38:06
 * @FilePath: /rustlings-semester-3-yaoxingpu/exercises/modules/modules1.rs
 * @Description:
 *
 */
// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.



pub mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
