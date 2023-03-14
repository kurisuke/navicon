mod conn;
mod library;
mod subsonic;
mod ui;

use std::{sync::mpsc::channel, thread};

use color_eyre::Result;
use config::Config;

use ui::event::UiEvent;

use crate::{library::Library, ui::Ui};

fn main() -> Result<()> {
    color_eyre::install()?;

    let (tx_library_request, rx_library_request) = channel();
    let (tx_ui_event, rx_ui_event) = channel();

    let ui_handler = thread::spawn(|| {
        let mut ui = Ui::new(tx_library_request, rx_ui_event).unwrap();
        ui.run().unwrap();
    });

    let config = Config::builder()
        .add_source(config::File::with_name("settings"))
        .build()?;

    let url: String = config.get("url")?;
    let user: String = config.get("user")?;
    let password: String = config.get("password")?;

    let conn = conn::Connection::new(url.clone(), user, password);
    tx_ui_event.send(UiEvent::AddLog(format!("ping: {}", conn.ping()?)))?;
    tx_ui_event.send(UiEvent::SetStatus(format!("connected to: {}", url)))?;

    let library_handler = thread::spawn(|| {
        let mut library = Library::new(conn, rx_library_request, tx_ui_event);
        library.run().unwrap();
    });

    library_handler.join().unwrap();
    ui_handler.join().unwrap();

    Ok(())
}
