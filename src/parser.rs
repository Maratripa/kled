fn clean_string(line: &str) -> String {
    let mut result = String::with_capacity(line.trim().len());

    let mut inside_double_quotes = false;
    let mut last_was_space = false;

    for c in line.trim().chars() {
        match c {
            '"' => {
                inside_double_quotes = !inside_double_quotes;
                last_was_space = false;
                result.push(c);
            }
            ' ' => {
                if inside_double_quotes || !last_was_space {
                    result.push(c);
                }
                last_was_space = true;
            }
            _ => {
                result.push(c);
                last_was_space = false;
            }
        }
    }

    result
}

pub fn parse_command(line: &str) -> Vec<String> {
    let line = clean_string(line);

    let mut ranges = Vec::new();
    let mut previous_start = 0;

    let mut inside_double_quotes = false;
    for (i, c) in line.char_indices() {
        match c {
            '"' => {
                if i - previous_start > 0 {
                    ranges.push(previous_start..i);
                }
                previous_start = i + 1;
                inside_double_quotes = !inside_double_quotes;
            }
            ' ' => {
                if !inside_double_quotes {
                    if i - previous_start > 0 {
                        ranges.push(previous_start..i);
                    }
                    previous_start = i + 1;
                }
            }
            _ => {
                continue;
            }
        }
    }
    if line.len() - previous_start > 0 {
        ranges.push(previous_start..line.len());
    }

    ranges
        .iter()
        .map(|r| line[r.start..r.end].to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_string() {
        let original_string = "  Hello  there,   \" General  Kenobi\" ";
        let result = clean_string(original_string);

        assert_eq!("Hello there, \" General  Kenobi\"", result);
    }

    #[test]
    fn test_space_and_quotes_parser() {
        let command = "replace  42 \"This is line  41.\" ";
        let result_vec = parse_command(command);
        let result: Vec<&str> = result_vec.iter().map(String::as_str).collect();
        assert_eq!(vec!["replace", "42", "This is line  41."], result);
    }
}
