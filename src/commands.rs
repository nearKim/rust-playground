#[derive(PartialEq, Debug)]
pub enum Command {
    Add(String, Option<String>),
    List(Option<bool>),
    Complete(u32),
    Remove(u32),
    Exit,
}

pub fn parse_command(input: &str) -> Result<Command, String> {
    todo!("Parse user input into a Command")
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
        assert_eq!(command, Command::List(Some(false)));
    }

    #[test]
    fn test_parse_list_command_completed() {
        let input = "list completed";
        let command = parse_command(input).expect("Failed to parse list command");
        assert_eq!(command, Command::List(Some(true)));
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
