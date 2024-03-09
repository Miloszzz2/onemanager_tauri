import { WebviewWindow } from "@tauri-apps/api/window";
export async function createSettingsWindow(programs: string[]) {
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
    webview.emit("tauri://created", function () {
        console.log("robie okno");
    });
    webview.once("tauri://error", function (e) {});
}
