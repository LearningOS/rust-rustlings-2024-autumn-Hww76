// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.


mod delicious_snacks {
    // TODO: Fix these use statements
    /*
    使用 pub use 重导出名称
    使用 use 关键字，将某个名称导入当前作用域后，
    这个名称在此作用域中就可以使用了，但它对此作用域之外还是私有的。
    如果想让其他人调用我们的代码时，也能够正常使用这个名称，就好像它本来就在当前作用域一样，
    那我们可以将 pub 和 use 合起来使用。
    这种技术被称为 “重导出（re-exporting）”：
        我们不仅将一个名称导入了当前作用域，还允许别人把它导入他们自己的作用域。
     */
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
