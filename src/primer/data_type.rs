pub mod number;

pub mod boolean;

pub mod char;

pub mod string;

pub mod vector;

pub mod custom;

pub mod conversion;

#[cfg(test)]
mod tests {
    #[test]
    fn example_literal() {
        // 带后缀的字面量，其类型在初始化时已经知道了。
        let x = 1u8;
        let y = 2u32;
        let z = 3f32;

        // 无后缀的字面量，其类型取决于如何使用它们。
        let i = 1;
        let f = 1.0;

        // `size_of_val` 返回一个变量所占的字节数
        println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
        println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
        println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
        println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
        println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    }

    #[test]
    fn example_inference() {
        // 因为有类型说明，编译器知道 `elem` 的类型是 u8。
        let elem = 5u8;

        // 创建一个空向量（vector，即不定长的，可以增长的数组）。
        let mut vec = Vec::new();
        // 现在编译器还不知道 `vec` 的具体类型，只知道它是某种东西构成的向量（`Vec<_>`）

        // 在向量中插入 `elem`。
        vec.push(elem);
        // 啊哈！现在编译器知道 `vec` 是 u8 的向量了（`Vec<u8>`）。
        // 试一试 ^ 注释掉 `vec.push(elem)` 这一行。

        println!("{:?}", vec);
    }

    #[test]
    fn example_alia() {
        // `NanoSecond` 是 `u64` 的新名字。
        type NanoSecond = u64;
        type Inch = u64;

        // 通过这个属性屏蔽警告。
        #[allow(non_camel_case_types)]
        type u64_t = u64;
        // 试一试 ^ 移除上面那个属性

        // `NanoSecond` = `Inch` = `u64_t` = `u64`.
        let nanoseconds: NanoSecond = 5 as u64_t;
        let inches: Inch = 2 as u64_t;

        // 注意类型别名*并不能*提供额外的类型安全，因为别名*并不是*新的类型。
        println!(
            "{} nanoseconds + {} inches = {} unit?",
            nanoseconds,
            inches,
            nanoseconds + inches
        );
    }
}
