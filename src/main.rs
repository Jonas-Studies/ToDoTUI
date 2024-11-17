mod task;
mod storage;

use task::Task;

fn main() {
    let tasks = if let Some(stored_tasks) = storage::get_tasks() {
        stored_tasks
    }
    else {
        vec![ Task::new(String::from("Programm schreiben")) ]
    };

    storage::set_tasks(& tasks);
}
