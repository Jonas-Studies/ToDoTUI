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

    while let Some(index_of_selected_task) = tui::select_one_task(& tasks) {
        tui::edit_one_task(& mut tasks[index_of_selected_task]);
    }

    storage::set_tasks(& tasks);
}
