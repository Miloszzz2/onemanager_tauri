<script lang="ts">
    import Switch from "../ui/switch/switch.svelte";
    import Label from "../ui/label/label.svelte";
    import { toggleMode } from "mode-watcher";
    import { mode } from "mode-watcher";
</script>

<div class="p-3 space-y-2">
    <Label class="text-gray-400">Theme</Label>
    <div class="flex items-center space-x-3">
        <Switch
            id="airplane-mode"
            checked={$mode == "dark" ? true : false}
            on:click={toggleMode}
        />
        <Label for="airplane-mode">Dark Mode</Label>
    </div>
</div>
