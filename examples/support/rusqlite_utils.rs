use super::Result;
use rusqlite::Connection;

pub fn create_schema(conn: &Connection) -> Result<()> {
	conn.execute(
		"CREATE TABLE IF NOT EXISTS project (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            name      TEXT
        ) STRICT",
		(), // empty list of parameters.
	)?;

	conn.execute(
		"CREATE TABLE IF NOT EXISTS task (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
						project_id INTEGER, 
            title      TEXT,
						desc       TEXT
        ) STRICT",
		(), // empty list of parameters.
	)?;

	Ok(())
}

pub fn seed_data(conn: &Connection) -> Result<()> {
	let suffixes = &["A", "B"];

	for suffix in suffixes {
		let project_name = format!("Project {suffix}");
		let mut stmt = conn.prepare("INSERT INTO project (name) VALUES (?1) RETURNING id")?;
		let project_id = stmt.query_row((&project_name,), |r| r.get::<_, i64>(0))?;

		for i in 1..=3 {
			let title = format!("Task {suffix}.{i}");
			let desc = format!("Description {suffix}.{i}");
			conn.execute(
				"INSERT INTO task (project_id, title, desc) VALUES (?1, ?2, ?3)",
				(project_id, &title, &desc),
			)?;
		}
	}

	Ok(())
}
