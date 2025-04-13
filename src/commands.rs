use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Command {
    Add(String, Option<String>),
    List(Option<String>),
    Complete(u32),
    Remove(u32),
    Exit,
}

trait CommandParseStrategy {
    fn parse(&self, input: &str) -> Result<Command, String>;
}

struct AddStrategy;
impl CommandParseStrategy for AddStrategy {
    fn parse(&self, input: &str) -> Result<Command, String> {
        let mut iter = input.split_whitespace();
        let _ = iter.next().unwrap();

        let mut title = String::new();
        let mut due_date = None;
        let mut found_due = false;

        for part in iter {
            if found_due {
                due_date = Some(part.to_string());
                break;
            } else if part == "--due" {
                found_due = true;
            } else {
                if !title.is_empty() {
                    title.push(' ');
                }
                title.push_str(part);
            }
        }

        if title.is_empty() {
            Err("Title is required for add command".to_string())
        } else {
            Ok(Command::Add(title, due_date))
        }
    }
}

struct ListStrategy;
impl CommandParseStrategy for ListStrategy {
    fn parse(&self, input: &str) -> Result<Command, String> {
        let mut iter = input.split_whitespace();

        let _ = iter.next().unwrap();

        let option = iter.next();
        if let Some(opt) = option {
            match opt {
                "pending" | "completed" => Ok(Command::List(Some(opt.to_string()))),
                _ => Err(format!(
                    "Invalid option '{}'. Expected 'pending' or 'completed'",
                    opt
                )),
            }
        } else {
            Ok(Command::List(None))
        }
    }
}

struct CompleteStrategy;
impl CommandParseStrategy for CompleteStrategy {
    fn parse(&self, input: &str) -> Result<Command, String> {
        let mut iter = input.split_whitespace();
        let _ = iter.next().unwrap();
        let id = iter
            .next()
            .ok_or("ID is required for complete command")?
            .parse::<u32>()
            .map_err(|_| "Invalid ID format")?;

        Ok(Command::Complete(id))
    }
}

struct RemoveStrategy;
impl CommandParseStrategy for RemoveStrategy {
    fn parse(&self, input: &str) -> Result<Command, String> {
        let mut iter = input.split_whitespace();
        let _ = iter.next().unwrap();
        let id = iter
            .next()
            .ok_or("ID is required for complete command")?
            .parse::<u32>()
            .map_err(|_| "Invalid ID format")?;

        Ok(Command::Remove(id))
    }
}

struct ExitStrategy;
impl CommandParseStrategy for ExitStrategy {
    fn parse(&self, _input: &str) -> Result<Command, String> {
        Ok(Command::Exit)
    }
}

pub fn parse_command(input: &str) -> Result<Command, String> {
    // 아래 코드가 안되는 이유:
    // Rust는 compile시 모든것들의 크기를 알아야 한다. 같은 trait을 구현해도, size는 다를 수 있다.
    // 이에 따라 동일한 크기의 원소들을 받아야 하는 Array의 제약조건을 만족시킬 수 없다.
    // 이 때 trait object를 사용한다.
    // let strategies = HashMap::from([
    //     ("add", AddStrategy),
    //     ("list", ListStrategy),
    //     ("complete", CompleteStrategy),
    //     ("remove", RemoveStrategy),
    //     ("exit", ExitStrategy),
    // ]);
    // Trait object는 2가지로 이뤄진 fat pointer다
    // 1.data를 가리키는 pointer
    // 2.vtable를 가리키는 pointer
    // trait object의 함수를 호출하면, Rust는 vtable을 찾아보고 맞는 implementation을 선택한 후 호출한다
    // 이 과정은 runtime에 이뤄지므로 dynamic dispatch. 약간의 runtime overhead로 flexibility를 확보.

    // Rust가 이거는 또 타입추론을 못함...
    // let strategies: HashMap<&str, Box<dyn CommandParseStrategy>> = HashMap::from([
    //     ("add", Box::new(AddStrategy)),
    //     ("list", Box::new(ListStrategy)),
    //     ("complete", Box::new(CompleteStrategy)),
    //     ("remove", Box::new(RemoveStrategy)),
    //     ("exit", Box::new(ExitStrategy)),
    // ]);

    let mut strategies: HashMap<&str, Box<dyn CommandParseStrategy>> = HashMap::new();
    strategies.insert("add", Box::new(AddStrategy));
    strategies.insert("list", Box::new(ListStrategy));
    strategies.insert("complete", Box::new(CompleteStrategy));
    strategies.insert("remove", Box::new(RemoveStrategy));
    strategies.insert("exit", Box::new(ExitStrategy));

    // The split_whitespace() method returns an iterator over substrings separated by whitespace.
    // This iterator is lazy,
    let cmd_str = match input.split_whitespace().next() {
        Some(cmd) => cmd,
        None => return Err("No command provided".to_string()),
    };

    match strategies.get(cmd_str) {
        Some(strategy) => strategy.parse(input),
        None => Err(format!("Unknown command: '{}'", cmd_str)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_add_command_with_due_date() {
        let input = "add Buy milk --due 2023-12-01";
        let command = parse_command(input).expect("Failed to parse add command");
        assert_eq!(
            command,
            Command::Add("Buy milk".to_string(), Some("2023-12-01".to_string()))
        );
    }

    #[test]
    fn test_parse_add_command_no_due_date() {
        let input = "add Write code";
        let command = parse_command(input).expect("Failed to parse add command");
        assert_eq!(command, Command::Add("Write code".to_string(), None));
    }

    #[test]
    fn test_parse_add_command_empty_description() {
        let input = "add";
        let result = parse_command(input);
        assert!(result.is_err()); // Should fail due to missing description
    }

    #[test]
    fn test_parse_list_command_all() {
        let input = "list";
        let command = parse_command(input).expect("Failed to parse list command");
        assert_eq!(command, Command::List(None));
    }

    #[test]
    fn test_parse_list_command_pending() {
        let input = "list pending";
        let command = parse_command(input).expect("Failed to parse list command");
        assert_eq!(command, Command::List(Some("pending".to_string())));
    }

    #[test]
    fn test_parse_list_command_completed() {
        let input = "list completed";
        let command = parse_command(input).expect("Failed to parse list command");
        assert_eq!(command, Command::List(Some("completed".to_string())));
    }

    #[test]
    fn test_parse_list_command_invalid_filter() {
        let input = "list invalid";
        let result = parse_command(input);
        assert!(result.is_err()); // Should fail for unknown filter
    }

    #[test]
    fn test_parse_complete_command_valid() {
        let input = "complete 42";
        let command = parse_command(input).expect("Failed to parse complete command");
        assert_eq!(command, Command::Complete(42));
    }

    #[test]
    fn test_parse_complete_command_no_id() {
        let input = "complete";
        let result = parse_command(input);
        assert!(result.is_err()); // Should fail due to missing ID
    }

    #[test]
    fn test_parse_complete_command_invalid_id() {
        let input = "complete abc";
        let result = parse_command(input);
        assert!(result.is_err()); // Should fail due to non-numeric ID
    }

    #[test]
    fn test_parse_remove_command_valid() {
        let input = "remove 7";
        let command = parse_command(input).expect("Failed to parse remove command");
        assert_eq!(command, Command::Remove(7));
    }

    #[test]
    fn test_parse_remove_command_no_id() {
        let input = "remove";
        let result = parse_command(input);
        assert!(result.is_err()); // Should fail due to missing ID
    }

    #[test]
    fn test_parse_exit_command() {
        let input = "exit";
        let command = parse_command(input).expect("Failed to parse exit command");
        assert_eq!(command, Command::Exit);
    }

    #[test]
    fn test_parse_empty_input() {
        let input = "";
        let result = parse_command(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_unknown_command() {
        let input = "foobar";
        let result = parse_command(input);
        assert!(result.is_err());
    }
}
