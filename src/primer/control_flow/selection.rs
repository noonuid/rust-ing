#[cfg(test)]
mod tests {
    #[test]
    fn example_if_else() {
        let n = 5;

        if n < 0 {
            print!("{} is negative", n);
        } else if n > 0 {
            print!("{} is positive", n);
        } else {
            print!("{} is zero", n);
        }

        let big_n = if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // 这个表达式返回一个 `i32` 类型。
            10 * n
        } else {
            println!(", and is a big number, half the number");

            // 这个表达式也必须返回一个 `i32` 类型。
            n / 2
            // 试一试 ^ 试着加上一个分号来结束这条表达式。
        };
        // ^ 不要忘记在这里加上一个分号！所有的 `let` 绑定都需要它。

        println!("{} -> {}", n, big_n);
    }

    #[test]
    fn example_match() {
        let number = 13;
        // 试一试 ^ 将不同的值赋给 `number`

        println!("Tell me about {}", number);
        match number {
            // 匹配单个值
            1 => println!("One!"),
            // 匹配多个值
            2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
            // 试一试 ^ 将 13 添加到质数列表中
            // 匹配一个闭区间范围
            13..=19 => println!("A teen"),
            // 处理其他情况
            _ => println!("Ain't special"),
            // 试一试 ^ 注释掉这个总括性的分支
        }

        let boolean = true;
        // match 也是一个表达式
        let binary = match boolean {
            // match 分支必须覆盖所有可能的值
            false => 0,
            true => 1,
            // 试一试 ^ 将其中一条分支注释掉
        };

        println!("{} -> {}", boolean, binary);
    }
}
