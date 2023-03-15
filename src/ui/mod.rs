pub mod event;
mod model;

use color_eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io,
    sync::mpsc::{Receiver, Sender},
    thread,
    time::Duration,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::library::{request::LibraryRequest, LibraryItemKey};

use self::event::{LibraryRequestResult, UiEvent};

const TICK: Duration = Duration::from_millis(200);

pub struct Ui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    app_state: AppState,
    tx_library_request: Sender<LibraryRequest>,
    rx_ui_event: Receiver<UiEvent>,
    redraw: bool,
}

impl Ui {
    pub fn new(
        tx_library_request: Sender<LibraryRequest>,
        rx_ui_event: Receiver<UiEvent>,
    ) -> Result<Ui> {
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
            tx_library_request,
            rx_ui_event,
            redraw: false,
        })
    }

    fn add_log(&mut self, msg: &str) {
        self.app_state.log.push_str(msg);
        self.app_state.log.push('\n');
    }

    fn set_status(&mut self, status: &str) {
        self.app_state.status = String::from(status);
    }

    fn set_library_view(&mut self, items: LibraryRequestResult) {
        self.app_state.library_view = items
            .into_iter()
            .map(|(id, item)| UiLibraryItem {
                id,
                text: item.to_string(),
            })
            .collect();
        self.app_state
            .library_view
            .sort_by(|a, b| a.text.cmp(&b.text));
    }

    pub fn run(&mut self) -> Result<()> {
        self.tx_library_request
            .send(LibraryRequest::GetChildren(LibraryItemKey::Root))?;

        loop {
            // user input events
            while crossterm::event::poll(Duration::from_secs(0))? {
                match crossterm::event::read()? {
                    Event::Key(key) => {
                        // keyboard input
                        if let KeyCode::Char('q') = key.code {
                            // shutdown
                            self.tx_library_request.send(LibraryRequest::Shutdown)?;
                            return Ok(());
                        }
                    }
                    Event::Resize(_, _) => {
                        // resized terminal, redraw
                        self.redraw = true;
                    }
                    _ => {}
                }
            }

            while let Ok(ui_event) = self.rx_ui_event.try_recv() {
                match ui_event {
                    UiEvent::LibraryGetChildrenComplete(_view_id, children_result) => {
                        self.set_library_view(children_result);
                    }
                    UiEvent::LibraryFindEntriesComplete(_, _) => todo!(),
                    UiEvent::AddLog(s) => {
                        self.add_log(&s);
                    }
                    UiEvent::SetStatus(s) => {
                        self.set_status(&s);
                    }
                }
                self.redraw = true;
            }

            if self.redraw {
                self.terminal.draw(|f| ui(f, &self.app_state))?;
                self.redraw = false;
            }

            thread::sleep(TICK);
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
    id: LibraryItemKey,
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
