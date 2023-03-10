mod conn;
mod library;
mod subsonic;
mod ui;

use std::{thread, time::Duration};

use color_eyre::Result;
use config::Config;

use crate::{
    library::{Item, Library},
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

    let mut library = Library::new();
    library.update_root(&conn)?;

    let artist_id = {
        let artist_ids = library.find_artist("matsushita");
        assert_eq!(artist_ids.len(), 1);
        artist_ids[0].to_string()
    };
    library.update_artist(&conn, &artist_id)?;

    let album_id = {
        let album_ids = library.find_album("first light");
        assert_eq!(album_ids.len(), 1);
        album_ids[0].to_string()
    };
    library.update_album(&conn, &album_id)?;

    let song_id = {
        let song_ids = library.find_song("really gone");
        assert_eq!(song_ids.len(), 1);
        song_ids[0].to_string()
    };
    if let Some(Item::Song(song)) = library.get_item(&song_id) {
        ui.add_log(&format!("{}", song))?;
    }

    let find_id = {
        let find_ids = library.find_entry("all i have");
        assert_eq!(find_ids.len(), 1);
        find_ids[0].to_string()
    };
    if let Some(Item::Song(song)) = library.get_item(&find_id) {
        ui.add_log(&format!("{}", song))?;
    }

    thread::sleep(Duration::from_secs(5));

    Ok(())
}
