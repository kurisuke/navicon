mod conn;
mod library;
mod subsonic;
mod ui;

use color_eyre::Result;
use config::Config;
use library::LibraryItemKey;

use crate::{
    library::{Library, LibraryItem},
    ui::Ui,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut ui = Ui::new()?;

    let config = Config::builder()
        .add_source(config::File::with_name("settings"))
        .build()?;

    let url: String = config.get("url")?;
    let user: String = config.get("user")?;
    let password: String = config.get("password")?;

    let conn = conn::Connection::new(url.clone(), user, password);
    ui.add_log(&format!("ping: {}", conn.ping()?))?;
    ui.set_status(&format!("connected to: {}", url))?;

    let mut library = Library::new(conn);
    ui.set_library_view(&mut library, &LibraryItemKey::Root)?;

    ui.wait_exit()?;

    Ok(())
}
