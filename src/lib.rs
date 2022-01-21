use serde_json::Value;

pub struct Arguments {
    compact: bool,
}

impl Arguments {
    pub fn new(compact: bool) -> Arguments {
        Arguments { compact }
    }
}

pub fn process_json(args: Arguments, buffer: String) -> anyhow::Result<String> {
    if buffer.is_empty() {
        return Ok(String::new());
    }

    // parse json
    let obj: Value = serde_json::from_str(&buffer)?;

    Ok(match args.compact {
        true => {
            // compact print
            serde_json::to_string(&obj)
        }
        false => {
            // pretty print
            serde_json::to_string_pretty(&obj)
        }
    }?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact() {
        let str = process_json(
            Arguments { compact: true },
            String::from("{\"hello\": {\"world\": [2, 4, 5]}} "),
        )
        .unwrap();
        assert_eq!(str, "{\"hello\":{\"world\":[2,4,5]}}");
    }

    #[test]
    fn test_pretty() {
        let str = process_json(
            Arguments { compact: false },
            String::from("{\"hello\": {\"world\": [2, 4, 5]}} "),
        )
        .unwrap();
        assert_eq!(
            str,
            "{\n  \"hello\": {\n    \"world\": [\n      2,\n      4,\n      5\n    ]\n  }\n}"
        );
    }

    #[test]
    fn test_empty_string() {
        process_json(Arguments { compact: false }, String::from("")).unwrap();
    }

    #[test]
    fn test_empty_object() {
        let str = process_json(Arguments { compact: true }, String::from("{ } ")).unwrap();
        assert_eq!(str, "{}");
    }
}
