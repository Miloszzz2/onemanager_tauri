import Database from "tauri-plugin-sql-api";

export const db_pool = async () => {
    const db = await Database.load(import.meta.env.VITE_DATABASE_URL);
    return db
};