#[derive(PartialEq, Debug)]
pub enum Command {
    Add(String, Option<String>),
    List(Option<String>),
    Complete(u32),
    Remove(u32),
    Exit,
}

pub fn parse_command(input: &str) -> Result<Command, String> {
    let cmd_type = input.split_whitespace().next().unwrap().to_lowercase();
    let cmd_content_list: Vec<_> = 
        input.split_whitespace()
            .skip(1)
            .take_while(|word| !word.contains("--"))
            .map(|word| word.to_string())
            .collect();
    let cmd_content = cmd_content_list.join(" ");
    let cmd_sub_list: Vec<_> = 
        input.split_whitespace().rev()
            .take_while(|word| !word.contains("--"))
            .map(|word| word.to_string())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
    let cmd_sub = cmd_sub_list.join(" ");

    match cmd_type.as_str() {
        "add" => {
            if (cmd_content.is_empty()) {
                return Err("".to_string());
            }
            else {
                return Ok(Command::Add(cmd_content, Some(cmd_sub)));
            }
        }

        "list" => {
            if !cmd_content.is_empty() {
                if (cmd_content != "completed") && (cmd_content != "pending") {
                    return Err("Invalid list filter".to_string());
                }
            }
            return Ok(Command::List(Some(cmd_content)));
        }

        "complete" => {
            let id = cmd_content.parse::<u32>()
                .map_err(|_| "Invalid number".to_string())?;
            return Ok(Command::Complete(id));
        }

        "remove" => {
            let id = cmd_content.parse::<u32>()
                .map_err(|_| "Invalid number".to_string())?;
            return Ok(Command::Remove(id));
        }

        "exit" => {
            return Ok(Command::Exit);
        }

        _ => {
            println!("Command No match");
            return Err("".to_string());
        }
    }

    return Ok(Command::Complete(0));
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
