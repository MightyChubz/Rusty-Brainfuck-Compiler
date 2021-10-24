pub mod intermediate;

pub fn generate_output(input: &String) -> String {
    fn indent(scope: &u32) -> String {
            let tabs = String::from("\t");
            tabs.repeat(*scope as usize)
    }

    let mut scope: u32 = 1;
    let mut builder: String = include_str!("preface.c").to_string();
    for line in input.lines() {
        let mut indentation: String = indent(&scope);
        let opt: Vec<&str> = line.split(" ").collect();

        match opt[0] {
            "add" => {
                if opt[1] == "1" {
                    builder.push_str(format!("{}(*i)++;\r\n", indentation).as_str());
                } else {
                    builder.push_str(format!("{}(*i) += {};\r\n", indentation, opt[1]).as_str());
                }
            },
            "sub" => {
                if opt[1] == "1" {
                    builder.push_str(format!("{}(*i)--;\r\n", indentation).as_str());
                } else {
                    builder.push_str(format!("{}(*i) -= {};\r\n", indentation, opt[1]).as_str());
                }
            },
            "lshift" => {
                if opt[1] == "1" {
                    builder.push_str(format!("{}i--;\r\n", indentation).as_str());
                } else {
                    builder.push_str(format!("{}i -= {};\r\n", indentation, opt[1]).as_str());
                }
            },
            "rshift" => {
                if opt[1] == "1" {
                    builder.push_str(format!("{}i++;\r\n", indentation).as_str());
                } else {
                    builder.push_str(format!("{}i += {};\r\n", indentation, opt[1]).as_str());
                }
            },
            "in" => {
                builder.push_str(format!("{}*i = getchar();\r\n", indentation).as_str());
            },
            "out" => {
                builder.push_str(format!("{}putchar(*i);\r\n", indentation).as_str());
            },
            "bloop" => {
                scope += 1;
                builder.push_str(format!("{}while (*i) {{\r\n", indentation).as_str());
            },
            "eloop" => {
                scope -= 1;
                indentation = indent(&scope);

                builder.push_str(format!("{}}}\r\n", indentation).as_str());
            },
            _ => ()
        }
    }

    builder.push_str("\treturn(0);\n");
    builder.push_str("}");
    
    builder
}