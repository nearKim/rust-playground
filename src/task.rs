use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{format, Formatter};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
    pub due_date: Option<String>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let check = if self.completed { "✓" } else { "✗" };
        /*
        `self.due_date`는 `&Option<String>` (reference).
        `as_ref()`는 이것을 `Option<&String>` 변환: It “moves the reference inside” the Option.
        `unwrap_or` 호출 시 `self.due_date`의 ownership을 요구하지만, `&self`로 빌려만 왔기에 불가.
        - `unwrap_or`는 Option을 소비하며 value를 (Some일 경우) Some 밖으로 move.
        - `self.due_date.unwrap_or(String::from(""))`는 shared reference인 `&Option<String>`을 ownership을 가지려고 시도
        - 이는 `self.due_date`을 struct 밖으로 move한다는 뜻이고, struct는 invalid state가 될 것이므로 Rust는 이를 금지: you can’t move something out of a borrowed reference
        - 이에 따라 compile error 발생: "cannot move out of `self.due_date` which is behind a shared reference"
        let due_date_string = self.due_date.as_ref().unwrap_or(&String::from(""));
        */
        let due_str = self
            .due_date
            .as_ref()
            .map(|date| format!("(Due: {})", date))
            .unwrap_or_default();

        /*
        1. `format!(...)`은 String을 생성하여 메모리에 할당.
        2. `.trim`이 호출되면 기존 String을 변경하지 않고, 기존 String의 일부(whitespace를 제거한)를 가리키는 string slice(&str)을 생성
        3. 이 &str이 result에 바인딩되지만, 문제는 1에서 생성한 String은 임시값이므로 어떠한 변수에도 할당되지 않
        4. 따라서 이 statement가 완료되면, Rust는 임시 String을 할당해제 -> result는 dangling reference
        let result = format!("[{}] {} {}", check, self.description, due_str).trim();
         */
        let result = format!("[{}] {} {}", check, self.description, due_str);
        f.write_str(result.trim())
    }
}

impl Task {
    pub fn new(id: u32, description: String, due_date: Option<String>) -> Self {
        Self {
            id,
            description,
            due_date,
            completed: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_task_creation() {
        let task = Task {
            id: 1,
            description: "Buy groceries".to_string(),
            completed: false,
            due_date: Some("2023-12-01".to_string()),
        };
        assert_eq!(task.id, 1);
        assert_eq!(task.description, "Buy groceries");
        assert_eq!(task.completed, false);
        assert_eq!(task.due_date, Some("2023-12-01".to_string()));
    }

    #[test]
    fn test_task_creation_empty_description() {
        let task = Task {
            id: 2,
            description: "".to_string(),
            completed: false,
            due_date: None,
        };
        assert_eq!(task.description, ""); // Should allow empty descriptions or reject based on design
    }

    #[test]
    fn test_task_to_string_pending() {
        let task = Task {
            id: 3,
            description: "Write tests".to_string(),
            completed: false,
            due_date: None,
        };
        assert_eq!(task.to_string(), "[✗] Write tests");
    }

    #[test]
    fn test_task_to_string_completed() {
        let task = Task {
            id: 4,
            description: "Finish project".to_string(),
            completed: true,
            due_date: Some("2023-12-01".to_string()),
        };
        assert_eq!(task.to_string(), "[✓] Finish project (Due: 2023-12-01)");
    }

    #[test]
    fn test_task_with_invalid_due_date_format() {
        let task = Task {
            id: 5,
            description: "Invalid date task".to_string(),
            completed: false,
            due_date: Some("not-a-date".to_string()),
        };
        // Assuming validation exists, this could panic or be handled gracefully
        assert_eq!(task.due_date, Some("not-a-date".to_string()));
    }
}
