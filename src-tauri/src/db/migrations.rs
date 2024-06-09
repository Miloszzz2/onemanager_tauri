use tauri_plugin_sql::{Migration, MigrationKind};
pub fn get_migrations() -> Vec<tauri_plugin_sql::Migration> {
    let migrations = vec![
        // Define your migrations here
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: include_str!("./migrations/tables.sql"),
            kind: MigrationKind::Up,
        },
    ];
    return migrations;
}
