import { WebviewWindow } from "@tauri-apps/api/window";
import { emit } from "@tauri-apps/api/event";
import type { apps } from "src/types/apps";
export async function CreateSettingsWindow(programs: apps[]) {
    const webview = new WebviewWindow("Settings", {
        url: "../settings.html",
        width: 1000,
        height: 600,
        title: "settings",
        transparent: true,
        decorations: false,
        alwaysOnTop: true,
        resizable: false,
    });
}
