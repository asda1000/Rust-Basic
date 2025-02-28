// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!



// 定义一个公开的枚举类型，用于表示不同的命令
pub enum Command {
    Uppercase,  // 将字符串转换为大写的命令
    Trim,       // 去除字符串首尾空白字符的命令
    Append(usize), // 向字符串末尾追加指定次数 "bar" 的命令，usize 表示追加次数
}

// 定义一个模块
mod my_module {
    // 从外部模块引入 Command 枚举
    use super::Command;

    // 定义 transformer 函数，接收一个包含 (String, Command) 元组的向量作为输入，返回一个字符串向量
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // 初始化一个空的字符串向量，用于存储处理后的字符串
        let mut output: Vec<String> = vec![];
        // 遍历输入向量中的每个元组
        for (string, command) in input.iter() {
            // 根据命令类型对字符串进行相应的处理
            let result = match command {
                Command::Uppercase => string.to_uppercase(), // 将字符串转换为大写
                Command::Trim => string.trim().to_string(), // 去除字符串首尾的空白字符
                Command::Append(n) => {
                    let mut new_string = string.clone();
                    // 循环追加指定次数的 "bar" 到字符串末尾
                    for _ in 0..*n {
                        new_string.push_str("bar");
                    }
                    new_string
                }
            };
            // 将处理后的字符串添加到输出向量中
            output.push(result);
        }
        // 明确返回 output 向量
        return output;
    }
}

// 测试模块，只有在测试环境下才会编译
#[cfg(test)]
mod tests {
    // 从外部模块引入 transformer 函数
    use super::my_module::transformer;
    // 从外部模块引入 Command 枚举
    use super::Command;

    // 定义一个测试函数
    #[test]
    fn it_works() {
        // 调用 transformer 函数，传入包含字符串和命令的向量
        let output = transformer(vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ]);
        // 断言输出的第一个元素是否符合预期
        assert_eq!(output[0], "HELLO");
        // 断言输出的第二个元素是否符合预期
        assert_eq!(output[1], "all roads lead to rome!");
        // 断言输出的第三个元素是否符合预期
        assert_eq!(output[2], "foobar");
        // 断言输出的第四个元素是否符合预期
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}