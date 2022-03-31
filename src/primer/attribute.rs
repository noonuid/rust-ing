#[cfg(test)]
mod tests {
    #[test]
    fn example_dead_code() {
        // `#[allow(dead_code)]` 属性可以禁用 `dead_code` lint
        #[allow(dead_code)]
        fn unused_function() {}

        #[allow(dead_code)]
        fn noisy_unused_function() {}
        // 改正 ^ 增加一个属性来消除警告
    }

    #[test]
    fn example_cfg() {
        #[allow(dead_code)]
        // 这个函数仅当目标系统是 Linux 的时候才会编译
        #[cfg(target_os = "linux")]
        fn are_you_on_linux() {
            println!("You are running linux!")
        }

        #[allow(dead_code)]
        // 而这个函数仅当目标系统 **不是** Linux 时才会编译
        #[cfg(not(target_os = "linux"))]
        fn are_you_on_linux() {
            println!("You are *not* running linux!")
        }

        {
            are_you_on_linux();

            println!("Are you sure?");
            if cfg!(target_os = "linux") {
                println!("Yes. It's definitely linux!");
            } else {
                println!("Yes. It's definitely *not* linux!");
            }
        }
    }
}
