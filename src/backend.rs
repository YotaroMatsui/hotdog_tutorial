use dioxus::prelude::*;


#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {

        let conn = rusqlite::Connection::open("hotdog.db").unwrap_or_else(|err| {
            eprintln!("Failed to open database: {}", err);
            panic!("Database initialization failed");
        });

        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );
            "#,
        ).unwrap_or_else(|err| {
            eprintln!("Failed to create table: {}", err);
            panic!("Table creation failed");
        });

        conn
    };
}

// Expose a `save_dog` endpoint on our server that takes an "image" parameter
#[server]
pub async fn save_dog(image: String) -> Result<(), ServerFnError> {
    DB.with(|db| {
        if let Err(err) = db.execute(
            "INSERT INTO dogs (url) VALUES (?1)",
            &[&image],
        ) {
            eprint!("Failed to save dog: {}", err);
            return Err(ServerFnError::new(err.to_string()));
        }

        Ok(())
    }
    );

    Ok(())
}

// Query the database and return the last 10 dogs and their url
#[server]
pub async fn list_dogs() -> Result<Vec<(usize, String)>, ServerFnError> {
    let dogs = DB.with(|f| {
        f.prepare("SELECT id, url FROM dogs ORDER BY id DESC LIMIT 10")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });

    Ok(dogs)
}

#[server]
pub async fn delete_dog(id: usize) -> Result<(), ServerFnError> {
    DB.with(|db| {
        if let Err(err) = db.execute(
            "DELETE FROM dogs WHERE id = ?1",
            &[&id],
        ) {
            eprint!("Failed to delete dog: {}", err);
            return Err(ServerFnError::new(err.to_string()));
        }

        Ok(())
    }
    );

    Ok(())
}