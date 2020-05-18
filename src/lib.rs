#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn cli_show_task() {
        use crate::cli::show_task;
        use colored::*;
        assert_eq!(show_task(0, 0, "test_task"), "test_task".red());
    }
}

pub mod cli {
    use chrono::{Datelike, Local};
    use colored::*;
    pub fn show_task(
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
}
