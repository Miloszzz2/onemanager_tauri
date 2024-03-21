<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { listen } from "@tauri-apps/api/event";
    import type { current_song } from "src/types/current_song";
    import { onMount } from "svelte";
    import Play from "lucide-svelte/icons/play";
    import SkipForward from "lucide-svelte/icons/skip-forward";
    import SkipBack from "lucide-svelte/icons/skip-back";
    $: current_song_value = "";
    onMount(() => {
        if (current_song_value == "") {
            invoke("current_music");
        }
        listen("current_song", (e: current_song) => {
            current_song_value = e.payload.message;
        });
    });
</script>

{#if current_song_value != ""}
    <div class="text-center pt-6 pb-3 flex items-center justify-center gap-2">
        <img src="integrations/spotify.png" alt="spotify" class="h-12" />
        <div>
            <h1 class="mb-2 px-4 text-xl font-semibold tracking-tighttext-2xl">
                {current_song_value}
            </h1>
            <div class="flex justify-center items-center gap-3 pt-2">
                <SkipBack class="cursor-pointer" />
                <Play class="cursor-pointer" />
                <SkipForward class="cursor-pointer" />
            </div>
        </div>
    </div>
{/if}
