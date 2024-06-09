import Settings from "./Settings.svelte";
import "../../styles.css";
const app = new Settings({
    target: <HTMLElement>document.getElementById("app"),
});

export default app;
