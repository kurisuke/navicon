use color_eyre::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::library::{Library, LibraryEntryKey};

pub struct Ui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    app_state: AppState,
}

impl Ui {
    pub(crate) fn new() -> Result<Ui> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let app_state = AppState {
            status: String::new(),
            log: String::new(),
            library_view: vec![],
        };

        terminal.draw(|f| ui(f, &app_state))?;

        Ok(Ui {
            terminal,
            app_state,
        })
    }

    pub(crate) fn add_log(&mut self, msg: &str) -> Result<()> {
        self.app_state.log.push_str(msg);
        self.app_state.log.push('\n');

        self.terminal.draw(|f| ui(f, &self.app_state))?;

        Ok(())
    }

    pub(crate) fn set_status(&mut self, status: &str) -> Result<()> {
        self.app_state.status = String::from(status);

        self.terminal.draw(|f| ui(f, &self.app_state))?;

        Ok(())
    }

    pub(crate) fn set_library_view(
        &mut self,
        library: &Library,
        parent: Option<LibraryEntryKey>,
    ) -> Result<()> {
        let entries = library.get_children(parent);
        self.app_state.library_view = entries
            .into_iter()
            .filter_map(|e| {
                library.get_item(&e).map(|item| UiLibraryItem {
                    id: e.clone(),
                    text: format!("{}", item),
                })
            })
            .collect();
        self.app_state
            .library_view
            .sort_by(|a, b| a.text.cmp(&b.text));

        self.terminal.draw(|f| ui(f, &self.app_state))?;

        Ok(())
    }

    pub(crate) fn wait_exit(&mut self) -> Result<()> {
        loop {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }

            self.terminal.draw(|f| ui(f, &self.app_state))?;
        }
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        self.terminal.show_cursor().unwrap();
    }
}

struct AppState {
    status: String,
    log: String,
    library_view: Vec<UiLibraryItem>,
}

struct UiLibraryItem {
    id: String,
    text: String,
}

fn ui<B: Backend>(f: &mut Frame<B>, app_state: &AppState) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(70),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(size);

    let create_block = |title| {
        Block::default().borders(Borders::ALL).title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
    };

    let status = Paragraph::new(Span::raw(&app_state.status)).block(create_block("navicon"));
    f.render_widget(status, chunks[0]);

    let items: Vec<_> = app_state
        .library_view
        .iter()
        .map(|item| ListItem::new(item.text.as_str()))
        .collect();
    let library_view = List::new(items).block(create_block("Library"));
    f.render_widget(library_view, chunks[1]);

    let log = Paragraph::new(Text::raw(&app_state.log))
        .block(create_block("Log"))
        .wrap(Wrap { trim: true });
    f.render_widget(log, chunks[2]);
}
