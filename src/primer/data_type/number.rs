#[cfg(test)]
mod tests {
    #[test]
    fn example_number() {
        // 整数字面量，下划线改善可读性
        let num_i32 = 1_000;
        println!("i32 literal is {}", num_i32);

        // 浮点数字面量，下划线改善可读性
        let num_f64 = 0.000_001;
        println!("f64 literal is {}", num_f64);
    }
}
