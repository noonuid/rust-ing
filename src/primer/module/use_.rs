#[allow(unused_imports)]
// 将 `deeply::nested::function` 路径绑定到 `other_function`。
use deeply::nested::function as other_function;

#[allow(dead_code)]
fn function() {
    println!("called `function()`");
}

#[allow(dead_code)]
mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_use() {
        // 更容易访问 `deeply::nested::funcion`
        other_function();

        println!("Entering block");
        {
            // 这和 `use deeply::nested::function as function` 等价。
            // 此 `function()` 将遮蔽外部的同名函数。
            use deeply::nested::function;
            function();

            // `use` 绑定拥有局部作用域。在这个例子中，`function()`
            // 的遮蔽只存在在这个代码块中。
            println!("Leaving block");
        }

        function();
    }
}
