#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[test]
    fn example_struct() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        // 单元结构体
        struct Unit;

        // 元组结构体
        struct Pair(i32, f32);

        // 带有两个字段的结构体
        struct Point {
            x: f32,
            y: f32,
        }

        // 结构体可以作为另一个结构体的字段
        #[allow(dead_code)]
        struct Rectangle {
            // 可以在空间中给定左上角和右下角在空间中的位置来指定矩形。
            top_left: Point,
            bottom_right: Point,
        }

        {
            // 使用简单的写法初始化字段，并创建结构体
            let name = String::from("Peter");
            let age = 27;
            let peter = Person { name, age };

            // 以 Debug 方式打印结构体
            println!("{:?}", peter);

            // 实例化结构体 `Point`
            let point: Point = Point { x: 10.3, y: 0.4 };

            // 访问 point 的字段
            println!("point coordinates: ({}, {})", point.x, point.y);

            // 使用结构体更新语法创建新的 point，
            // 这样可以用到之前的 point 的字段
            let bottom_right = Point { x: 5.2, ..point };

            // `new_point.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
            println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

            // 使用 `let` 绑定来解构 point
            let Point {
                x: left_edge,
                y: top_edge,
            } = point;

            let _rectangle = Rectangle {
                // 结构体的实例化也是一个表达式
                top_left: Point {
                    x: left_edge,
                    y: top_edge,
                },
                bottom_right: bottom_right,
            };

            // 实例化一个单元结构体
            let _unit = Unit;

            // 实例化一个元组结构体
            let pair = Pair(1, 0.1);

            // 访问元组结构体的字段
            println!("pair contains {:?} and {:?}", pair.0, pair.1);

            // 解构一个元组结构体
            let Pair(integer, decimal) = pair;

            println!("pair contains {:?} and {:?}", integer, decimal);
        }
    }

    #[test]
    fn example_enum() {
        // 创建一个 `enum`（枚举）来对 web 事件分类。注意变量名和类型共同指定了 `enum`
        // 取值的种类：`PageLoad` 不等于 `PageUnload`，`KeyPress(char)` 不等于
        // `Paste(String)`。各个取值不同，互相独立。
        enum WebEvent {
            // 一个 `enum` 可以是单元结构体（称为 `unit-like` 或 `unit`），
            PageLoad,
            PageUnload,
            // 或者一个元组结构体，
            KeyPress(char),
            Paste(String),
            // 或者一个普通的结构体。
            Click { x: i64, y: i64 },
        }

        // 此函数将一个 `WebEvent` enum 作为参数，无返回值。
        fn inspect(event: WebEvent) {
            match event {
                WebEvent::PageLoad => println!("page loaded"),
                WebEvent::PageUnload => println!("page unloaded"),
                // 从 `enum` 里解构出 `c`。
                WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
                WebEvent::Paste(s) => println!("pasted \"{}\".", s),
                // 把 `Click` 解构给 `x` and `y`。
                WebEvent::Click { x, y } => {
                    println!("clicked at x={}, y={}.", x, y);
                }
            }
        }

        {
            let pressed = WebEvent::KeyPress('x');
            // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`。
            let pasted = WebEvent::Paste("my text".to_owned());
            let click = WebEvent::Click { x: 20, y: 80 };
            let load = WebEvent::PageLoad;
            let unload = WebEvent::PageUnload;

            inspect(pressed);
            inspect(pasted);
            inspect(click);
            inspect(load);
            inspect(unload);
        }
    }
}
