use ratatui::{DefaultTerminal, Frame};

mod model;
mod view;

use model::task::Task;
use view::taskWidget::TaskWidget;

use crate::view::taskWidget::TaskApp;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| TaskApp::new().run(terminal))?;
    Ok(())
}

// fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
//     loop {
//         terminal.draw(render)?;
//         if crossterm::event::read()?.is_key_press() {
//             break Ok(());
//         }
//     }
// }

// fn render(frame: &mut Frame) {
//     let task = Task::new("My Task".to_string(), "This is a sample task.".to_string());
//     let task_widget = TaskWidget::new(task);
//     frame.render_widget(task_widget, frame.area());
// }