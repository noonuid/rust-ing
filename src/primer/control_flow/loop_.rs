#[cfg(test)]
mod tests {
    #[test]
    fn example_loop() {
        let mut sum = 0;
        let mut i = 0;

        loop {
            i = i + 1;
            sum = sum + i;

            if i == 100 {
                break;
            }
        }

        println!("1 + 2 + ... + 100 = {}", sum);
    }

    #[allow(unreachable_code)]
    #[allow(unused_labels)]
    #[test]
    fn example_nested_loop() {
        'outer: loop {
            println!("Entered the outer loop");

            'inner: loop {
                println!("Entered the inner loop");

                // 这只是中断内部的循环
                //break;

                // 这会中断外层循环
                break 'outer;
            }

            println!("This point will never be reached");
        }

        println!("Exited the outer loop");
    }

    #[test]
    fn example_loop_return() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        assert_eq!(result, 20);
    }

    #[test]
    fn example_while() {
        // 计数器变量
        let mut n = 1;

        // 当 `n` 小于 101 时循环
        while n < 101 {
            if n % 15 == 0 {
                println!("fizzbuzz");
            } else if n % 3 == 0 {
                println!("fizz");
            } else if n % 5 == 0 {
                println!("buzz");
            } else {
                println!("{}", n);
            }

            // 计数器值加 1
            n += 1;
        }
    }

    #[test]
    fn example_for() {
        // `n` 将在每次迭代中分别取 1, 2, ..., 100
        for n in 1..101 {
            if n % 15 == 0 {
                println!("fizzbuzz");
            } else if n % 3 == 0 {
                println!("fizz");
            } else if n % 5 == 0 {
                println!("buzz");
            } else {
                println!("{}", n);
            }
        }
    }

    #[test]
    fn example_for_iter() {
        let names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter() {
            match name {
                &"Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }
    }

    #[test]
    fn example_for_into_iter() {
        let names = vec!["Bob", "Frank", "Ferris"];

        for name in names.into_iter() {
            match name {
                "Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }
    }

    #[test]
    fn example_for_iter_mut() {
        let mut names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }
        println!("names: {:?}", names);
    }
}
