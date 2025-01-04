mod task;
mod storage;
mod tui;

use task::Task;

fn main() {
    let mut tasks = vec![ Task::new(String::from("Add new task ...")) ];

    if let Some(stored_tasks) = storage::get_tasks() {
        tasks.extend(stored_tasks);
    }

    while let Some(index_of_selected_task) = tui::select_one_task(& tasks) {
        if index_of_selected_task == 0 {
            let new_task = Task::new(String::new());
            
            if let Some(new_task) = tui::edit_one_task(new_task) {
                tasks.insert(1, new_task);
            }
        }
        else {
            if let Some(edited_task) = tui::edit_one_task(tasks[index_of_selected_task].clone()) {
                tasks[index_of_selected_task] = edited_task;
            }
            else {
                tasks.remove(index_of_selected_task);
            }
        }
    }

    storage::set_tasks(& tasks.split_off(1));
}
