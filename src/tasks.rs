use core::ops::Deref;

use crate::task::Task;

pub struct Tasks {
    // It is assumed that the content-vector is always ordered so that the unfinished tasks come before
    // the finished ones
    content: Vec<Task>
}

impl Tasks {
    pub fn from_ordered(tasks: Vec<Task>) -> Self {
        Self{ content: tasks }
    }
    pub fn reference_content(& self) -> & Vec<Task> {
        & self.content
    }
    pub fn reference_content_mutable(& mut self) -> & mut Vec<Task> {
        & mut self.content
    }
    pub fn move_task_to_second_half(& mut self, index_of_task_to_move: & usize) {
        let task_to_move = self.content[* index_of_task_to_move].clone();

        // When there's only one or no task at all there's nothing to be moved
        if self.content.len() <= 1 {
            return;
        }

        let index_of_last_task = self.content.len() - 1;

        // When the last task is not finished, there's no second half so the task has to be
        // appended at the end
        if self.content[index_of_last_task].is_finished() == false {
            self.content[* index_of_task_to_move] = self.content[index_of_last_task].clone();
            self.content[index_of_last_task] = task_to_move;
            return;
        }

        // The tasks, beginning with the task that has to be moved, get moved one closer to the
        // start until the first task thats finished is reached. Then, before this task the task
        // that has to be moved get's placed before it
        for index_of_current_task in * index_of_task_to_move .. index_of_last_task {
            let index_of_next_task = index_of_current_task + 1;

            // When the next task is finished, it is the current start of the second half. So the current
            // task has to become the new start.
            if self.content[index_of_next_task].is_finished() == true {
                self.content[index_of_current_task] = task_to_move;
                return;
            }
            // Otherwise the list get's moved one element further
            else {
                self.content[index_of_current_task] = self.content[index_of_next_task].clone();
            }
        }
    }
}

impl Deref for Tasks {
    type Target = Vec<Task>;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
