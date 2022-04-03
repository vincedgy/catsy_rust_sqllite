use rusqlite::{Connection, Result};

#[derive(Debug)]
struct CatColor {
    _id: i32,
    _name: String,
}

pub fn show_cat_colors() -> Result<()> {
    let conn = Connection::open("cats.db")?;

    let mut stmt = conn.prepare("SELECT id, name FROM cat_colors")?;
    let cat_color_iter = stmt.query_map([], |row| {
        Ok(CatColor {
            _id: row.get(0)?,
            _name: row.get(1)?,
        })
    })?;

    for cat_color in cat_color_iter {
        println!("Found cat {:?}", cat_color.unwrap());
    }
    Ok(())
}
