use chrono::{TimeDelta, Weekday};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Row, StatefulWidget, Table, TableState},
};

pub struct ReportViewState {
    table_state: TableState,
}

impl ReportViewState {
    pub fn new() -> Self {
        ReportViewState {
            table_state: TableState::default(),
        }
    }
}

pub struct ReportView {
    items: Vec<(String, Vec<TimeDelta>)>,
    weekday: Weekday,
}

// TODO: add compact view (today + task name)
impl ReportView {
    pub fn new(items: Vec<(String, Vec<TimeDelta>)>, weekday: Weekday) -> Self {
        ReportView { items, weekday }
    }

    fn make_header_row<'a>(today: usize) -> Row<'a> {
        let days = [
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ];

        let mut cells: Vec<Line> = Vec::new();

        for day in &days {
            let line = Line::from(day.to_string()).right_aligned();
            cells.push(line);
        }

        cells[today] = Span::from("Today")
            .bold()
            .into_right_aligned_line();

        cells.push(Line::from("Task Name"));
        Row::new(cells).gray().reversed()
    }

    fn make_table_row<'a>(
        task_name: &'a str,
        daily_times: &'a [TimeDelta],
        today: usize,
    ) -> Row<'a> {
        let mut cells = Vec::new();

        for (i, time) in daily_times.iter().enumerate() {
            let line = Line::from(delta_to_string(time)).right_aligned();

            if i == today {
                cells.push(line.bold());
            } else if i > today {
                cells.push(line.gray());
            } else {
                cells.push(line);
            }
        }

        cells.push(Line::from(task_name.to_string()));

        Row::new(cells)
    }

    fn make_table_rows<'a>(items: &'a [(String, Vec<TimeDelta>)], today: usize) -> Vec<Row<'a>> {
        let mut rows: Vec<Row> = Vec::new();

        for (index, (task_name, daily_times)) in items.iter().enumerate() {
            let mut row = Self::make_table_row(task_name, daily_times, today);
            if (index % 2) == 0 {
                row = row.style(Style::default().bg(Color::DarkGray));
            }
            rows.push(row);
        }
        rows
    }

    fn render_table(&self, area: Rect, buf: &mut Buffer, state: &mut ReportViewState) {
        let today = self.weekday.num_days_from_monday() as usize;
        let header = Self::make_header_row(today);
        let rows = Self::make_table_rows(&self.items, today);

        let widths = [
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Fill(1),
        ];

        let table = Table::new(rows, widths)
            .flex(ratatui::layout::Flex::Start)
            .header(header)
            .column_spacing(1);
        table.render(area, buf, &mut state.table_state);
    }
}

impl StatefulWidget for ReportView {
    type State = ReportViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.render_table(area, buf, state);
    }
}

fn delta_to_string(delta: &TimeDelta) -> String {
    let total_seconds = delta.num_seconds();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let _seconds = total_seconds % 60;
    format!("{:02}:{:02}", hours, minutes)
}
