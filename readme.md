## Requirements

### Overview
Students are tasked with implementing and enhancing a command-line to-do list manager in Rust, starting from the provided skeleton code. The program manages tasks with unique IDs, descriptions, completion status, and optional due dates, persisting them to a JSON file (`todo.json`). The skeleton code provides basic functionality, and students will extend it according to the requirements below.

### Functional Requirements
- **Core Functionality**: The to-do list manager must support the following operations via a command-line interface:
    - Add new tasks with descriptions and optional due dates.
    - List all tasks, with an optional filter for pending or completed tasks.
    - Mark tasks as completed by their ID.
    - Remove tasks by their ID.
    - Exit the program, ensuring the task list is saved.
- **Persistence**: Load the task list from `todo.json` on startup and save it to the same file after modifications or on exit.

### Technical Requirements
- **Rust Concepts**:
    - Use Rust's ownership and borrowing rules to manage the task list effectively.
    - Implement proper error handling using `Result` and `Option` instead of relying on `expect`.
    - Define and use traits (e.g., `Display` for tasks) to customize behavior.
    - Utilize iterators and closures for task filtering and manipulation.
- **Code Organization**: Split the code into logical modules (e.g., `task`, `todo_list`, `file_io`) for clarity and maintainability.
- **Concurrency**: Implement a background thread to auto-save the task list to `todo.json` every 10 seconds, using `Arc` and `Mutex` for thread-safe access.
- **Testing**: Write unit tests for core functions (e.g., adding, completing, removing tasks) and at least one integration test for the command-line interface.
- **Documentation**: Include inline comments explaining key logic and a `README.md` with installation and usage instructions.

### Commands
The program must recognize and handle the following commands, entered via the command line:

- **`add <description> [--due <date>]`**
    - **Description**: Adds a new task to the list.
    - **Arguments**:
        - `<description>`: A string describing the task (e.g., "Finish homework").
        - `--due <date>`: An optional due date in a student-defined format (e.g., "YYYY-MM-DD").
    - **Behavior**: Assigns a unique, auto-incremented ID to the task and adds it to the list with `completed` set to `false`.
    - **Feedback**: Prints a confirmation (e.g., "Task added with ID 1").

- **`list [filter]`**
    - **Description**: Displays all tasks or a filtered subset.
    - **Arguments**:
        - `[filter]`: Optional. Can be `"pending"` (show only incomplete tasks) or `"completed"` (show only completed tasks). If omitted, show all tasks.
    - **Behavior**: Prints each task’s ID, description, completion status, and due date (if set) in a readable format.
    - **Feedback**: Lists tasks or indicates if no tasks match the filter (e.g., "No pending tasks").

- **`complete <id>`**
    - **Description**: Marks a task as completed.
    - **Arguments**:
        - `<id>`: The integer ID of the task to mark as completed.
    - **Behavior**: Updates the task’s `completed` field to `true` if the ID exists.
    - **Feedback**: Confirms the action (e.g., "Task 2 completed") or indicates if the ID is invalid (e.g., "Task not found").

- **`remove <id>`**
    - **Description**: Deletes a task from the list.
    - **Arguments**:
        - `<id>`: The integer ID of the task to remove.
    - **Behavior**: Removes the task with the specified ID if it exists.
    - **Feedback**: Confirms the action (e.g., "Task 3 removed") or indicates if the ID is invalid.

- **`exit`**
    - **Description**: Exits the program.
    - **Arguments**: None.
    - **Behavior**: Saves the current task list to `todo.json` and terminates gracefully.
    - **Feedback**: Prints a goodbye message (e.g., "Exiting...").

### Enhancements from Skeleton
Students must extend the provided skeleton code by:
- Replacing `expect` calls with proper error handling for file I/O and user inputs, providing informative error messages.
- Adding the auto-save background thread as described in the technical requirements.
- Including the testing and documentation requirements outlined above.

### Additional Expectations
- **Input Validation**: Handle invalid commands or arguments gracefully (e.g., "Unknown command" or "Invalid ID").
- **User Feedback**: Provide clear feedback after each operation to confirm success or explain failures.
- **Task Management**: Ensure task IDs remain unique and are auto-incremented using the `next_id` field from the skeleton.
