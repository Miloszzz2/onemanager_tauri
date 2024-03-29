<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import { logout } from "$services/auth";
    import { WebviewWindow, appWindow } from "@tauri-apps/api/window";
</script>

<div class="p-3 !overflow-y-auto h-full">
    <Button
        on:click={async () => {
            logout();
            await WebviewWindow.getByLabel("main")?.hide();
            await WebviewWindow.getByLabel("Settings")?.hide();
            await WebviewWindow.getByLabel("Boarding")?.show();
        }}>Logout</Button
    >
</div>
