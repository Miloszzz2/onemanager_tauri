import "./styles.css";
import Settings from "./Settings.svelte";

const app = new Settings({
    target: <HTMLElement>document.getElementById("app"),
});

export default app;
