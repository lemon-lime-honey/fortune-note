use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};

pub type DbPool = Arc<Mutex<Connection>>;

pub fn init_db(path: &str) -> Result<DbPool> {
    let conn = if path == ":memory:" {
        Connection::open_in_memory()?
    } else {
        if let Some(parent) = std::path::Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                let _ = std::fs::create_dir_all(parent);
            }
        }
        Connection::open(path)?
    };

    configure_connection(&conn)?;
    apply_migrations(&conn)?;

    Ok(Arc::new(Mutex::new(conn)))
}

fn configure_connection(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        PRAGMA journal_mode = DELETE;
        PRAGMA synchronous = FULL;
        PRAGMA foreign_keys = ON;
        PRAGMA busy_timeout = 5000;
        "#,
    )?;
    Ok(())
}

fn apply_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL
        );
        "#,
    )?;

    let current_version: i32 = conn.query_row(
        "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
        [],
        |row| row.get(0),
    )?;

    if current_version < 1 {
        conn.execute_batch(
            r#"
            BEGIN TRANSACTION;

            CREATE TABLE profiles (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                gender TEXT NOT NULL,
                birth_year INTEGER NOT NULL,
                birth_month INTEGER NOT NULL,
                birth_day INTEGER NOT NULL,
                birth_hour INTEGER NOT NULL,
                birth_minute INTEGER NOT NULL,
                latitude REAL NOT NULL,
                longitude REAL NOT NULL,
                timezone_id TEXT NOT NULL,
                notes TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE INDEX idx_profiles_name ON profiles(name);
            CREATE INDEX idx_profiles_created ON profiles(created_at DESC);

            CREATE TABLE horary_questions (
                id TEXT PRIMARY KEY NOT NULL,
                title TEXT NOT NULL,
                category TEXT NOT NULL,
                question_text TEXT NOT NULL,
                query_year INTEGER NOT NULL,
                query_month INTEGER NOT NULL,
                query_day INTEGER NOT NULL,
                query_hour INTEGER NOT NULL,
                query_minute INTEGER NOT NULL,
                query_second REAL NOT NULL,
                latitude REAL NOT NULL,
                longitude REAL NOT NULL,
                timezone_id TEXT NOT NULL,
                querent_profile_id TEXT REFERENCES profiles(id) ON DELETE SET NULL,
                result_notes TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE INDEX idx_horary_created ON horary_questions(created_at DESC);
            CREATE INDEX idx_horary_querent ON horary_questions(querent_profile_id);

            INSERT INTO schema_migrations (version, applied_at)
            VALUES (1, datetime('now'));

            COMMIT;
            "#,
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_db_in_memory() {
        let pool = init_db(":memory:").expect("DB initialization should succeed");
        let conn = pool.lock().unwrap();

        let count: i64 = conn
            .query_row("SELECT count(*) FROM schema_migrations", [], |row| row.get(0))
            .expect("query should succeed");
        assert_eq!(count, 1);
    }
}
