#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn cli_show_task() {
        use crate::cli::*;
        use colored::*;
        assert_eq!(task_todo(0, 0, "todo_task"), "todo_task".red());
        show_task_todo(0, 0, "todo_task");
        assert_eq!(task_done("done_task"), "done_task".green());
        show_task_done("done_task");
    }
}

pub mod cli {
    use chrono::{Datelike, Local};
    use colored::*;
    pub fn task_todo(
        task_begin_week: u32,
        task_begin_week_day: u32,
        task_desc: &str,
    ) -> colored::ColoredString {
        let now = Local::now();
        if (now.iso_week().week() * 7 + now.weekday().num_days_from_sunday())
            >= (task_begin_week * 7 + task_begin_week_day)
        {
            task_desc.red()
        } else {
            task_desc.blue()
        }
    }

    pub fn task_done(task_desc: &str) -> colored::ColoredString {
        task_desc.green()
    }
    pub fn show_task_todo(task_begin_week: u32, task_begin_week_day: u32, task_desc: &str) {
        println!(
            "{}",
            task_todo(task_begin_week, task_begin_week_day, task_desc)
        );
    }
    pub fn show_task_done(task_desc: &str) {
        println!("{}", task_done(task_desc));
    }
}
