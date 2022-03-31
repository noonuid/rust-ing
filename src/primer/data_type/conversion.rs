#[cfg(test)]
mod tests {
    #[allow(overflowing_literals)]
    #[test]
    fn example_integer_to_integer() {
        // 当把任何类型转换为无符号类型 T 时，会不断加上或减去 (std::T::MAX + 1)
        // 直到值位于新类型 T 的范围内。

        // 1000 已经在 u16 的范围内
        println!("1000 as a u16 is: {}", 1000 as u16);

        // 1000 - 256 - 256 - 256 = 232
        // 事实上的处理方式是：从最低有效位（LSB，least significant bits）开始保留
        // 8 位，然后剩余位置，直到最高有效位（MSB，most significant bit）都被抛弃。
        // 译注：MSB 就是二进制的最高位，LSB 就是二进制的最低位，按日常书写习惯就是
        // 最左边一位和最右边一位。
        println!("1000 as a u8 is : {}", 1000 as u8);
        // -1 + 256 = 255
        println!("  -1 as a u8 is : {}", (-1i8) as u8);

        // 对正数，这就和取模一样。
        println!("1000 mod 256 is : {}", 1000 % 256);

        // 当转换到有符号类型时，（位操作的）结果就和 “先转换到对应的无符号类型，
        // 如果 MSB 是 1，则该值为负” 是一样的。

        // 当然如果数值已经在目标类型的范围内，就直接把它放进去。
        println!(" 128 as a i16 is: {}", 128 as i16);
        // 128 转成 u8 还是 128，但转到 i8 相当于给 128 取八位的二进制补码，其值是：
        println!(" 128 as a i8 is : {}", 128 as i8);

        // 重复之前的例子
        // 1000 as u8 -> 232
        println!("1000 as a u8 is : {}", 1000 as u8);
        // 232 的二进制补码是 -24
        println!(" 232 as a i8 is : {}", 232 as i8);
    }

    #[test]
    fn example_float_to_integer() {
        let decimal = 65.4321_f32;

        // 错误！不提供隐式转换
        // let integer: u8 = decimal;
        // 改正 ^ 注释掉这一行

        // 可以显式转换
        let integer = decimal as u8;
        let character = integer as char;

        println!("Casting: {} -> {} -> {}", decimal, integer, character);
    }

    #[allow(dead_code)]
    #[test]
    fn example_from() {
        let my_str = "hello";
        let my_string = String::from(my_str);
        println!("my_str is {}, my_string is {}", my_str, my_string);

        use std::convert::From;

        #[derive(Debug)]
        struct Number {
            value: i32,
        }

        impl From<i32> for Number {
            fn from(item: i32) -> Self {
                Number { value: item }
            }
        }

        let num = Number::from(30);
        println!("My number is {:?}", num);
    }

    #[allow(dead_code)]
    #[test]
    fn example_into() {
        use std::convert::From;

        #[derive(Debug)]
        struct Number {
            value: i32,
        }

        impl From<i32> for Number {
            fn from(item: i32) -> Self {
                Number { value: item }
            }
        }

        let int = 5;
        // 试试删除类型说明
        let num: Number = int.into();
        println!("My number is {:?}", num);
    }
}
