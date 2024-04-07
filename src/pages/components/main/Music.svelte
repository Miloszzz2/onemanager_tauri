<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { emit, listen } from "@tauri-apps/api/event";
    import { Command } from "@tauri-apps/api/shell";
    import type { current_song } from "src/types/current_song";
    import { onMount } from "svelte";
    import Play from "lucide-svelte/icons/play";
    import SkipForward from "lucide-svelte/icons/skip-forward";
    import SkipBack from "lucide-svelte/icons/skip-back";
    $: current_song_value = "";
    const nextTrackCommand = Command.sidecar("./bin/nextTrack");
    const playCommand = Command.sidecar("./bin/play");
    const prevTrackCommand = Command.sidecar("./bin/prevTrack");
    onMount(() => {
        if (current_song_value == "") {
            console.log("pobieram");
            invoke("current_music");
            if (current_song_value == "") {
                current_song_value = localStorage.getItem("lastSong") as string;
            }
        }
        listen("current_song", (e: current_song) => {
            console.log("recieved");
            current_song_value = e.payload.message;
            localStorage.setItem("lastSong", current_song_value);
        });
    });
</script>

<div class="text-center pt-6 pb-3 flex items-center justify-evenly px-10">
    <img src="integrations/spotify.png" alt="spotify" class="h-12" />
    <div>
        {#if current_song_value != ""}
            <h1 class="mb-2 px-4 text-lg font-semibold tracking-tighttext-2xl">
                {current_song_value}
            </h1>{:else}
            <h1 class="mb-2 px-4 text-lg font-semibold tracking-tighttext-2xl">
                No current running song
            </h1>
        {/if}
        <div class="flex justify-center items-center gap-3 pt-2">
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <div
                on:click={async () => {
                    await prevTrackCommand.execute();
                }}
            >
                <SkipBack class="cursor-pointer" />
            </div>
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <div
                on:click={async () => {
                    await playCommand.execute();
                }}
            >
                <Play class="cursor-pointer" />
            </div>
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <div
                on:click={async () => {
                    await nextTrackCommand.execute();
                }}
            >
                <SkipForward class="cursor-pointer" />
            </div>
        </div>
    </div>
</div>
