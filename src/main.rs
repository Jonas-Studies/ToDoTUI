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
            let mut new_task = Task::new(String::new());
            
            tui::edit_one_task(& mut new_task);

            if new_task.get_name() != "" {
                tasks.insert(1, new_task);
            }
        }
        else {
            tui::edit_one_task(& mut tasks[index_of_selected_task]);
        }
    }

    storage::set_tasks(& tasks.split_off(1));
}
