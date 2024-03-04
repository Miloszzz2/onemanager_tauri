<script lang="ts">
    import { ModeWatcher } from "mode-watcher";
    import Button from "$lib/components/ui/button/button.svelte";
    import { onMount } from "svelte";
    import { emit, listen } from "@tauri-apps/api/event";
    /* Imports here */

    var programs: string[];
    type programs = {
        payload: {
            programs: string[];
        };
    };
    onMount(async () => {
        console.log("robie");
        await listen("click", (e: programs) => {
            programs = e.payload.programs;
            console.log(programs);
            emit("stopSending");
        });
    });
</script>

<Button>Get data</Button>
<h1 class="bg-red-500">Settings</h1>
<ModeWatcher />
