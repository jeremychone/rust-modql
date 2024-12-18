use sqlx::{sqlite::SqlitePool, Result};

pub async fn create_schema(pool: &SqlitePool) -> Result<()> {
	sqlx::query(
		"CREATE TABLE IF NOT EXISTS project (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            name      TEXT
        ) STRICT",
	)
	.execute(pool)
	.await?;

	sqlx::query(
		"CREATE TABLE IF NOT EXISTS task (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER, 
            title      TEXT,
            desc       TEXT
        ) STRICT",
	)
	.execute(pool)
	.await?;

	Ok(())
}

pub async fn seed_data(pool: &SqlitePool) -> Result<()> {
	let suffixes = &["A", "B"];

	for suffix in suffixes {
		let project_name = format!("Project {suffix}");
		let row: (i64,) = sqlx::query_as("INSERT INTO project (name) VALUES (?) RETURNING id")
			.bind(&project_name)
			.fetch_one(pool)
			.await?;
		let project_id = row.0;

		for i in 1..=3 {
			let title = format!("Task {suffix}.{i}");
			let desc = format!("Description {suffix}.{i}");
			sqlx::query("INSERT INTO task (project_id, title, desc) VALUES (?, ?, ?)")
				.bind(project_id)
				.bind(&title)
				.bind(&desc)
				.execute(pool)
				.await?;
		}
	}

	Ok(())
}
