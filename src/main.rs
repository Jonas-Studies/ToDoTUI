mod task;
mod storage;
mod tui;

use task::Task;

fn main() {
    let mut tasks = if let Some(stored_tasks) = storage::get_tasks() {
        stored_tasks
    }
    else {
        vec![ Task::new(String::from("Programm schreiben")) ]
    };

    tui::edit_one_task(& mut tasks[0]);

    storage::set_tasks(& tasks);
}
