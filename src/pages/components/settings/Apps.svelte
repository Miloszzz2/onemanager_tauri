<script lang="ts">
    import Label from "$lib/components/ui/label/label.svelte";
    import Switch from "$lib/components/ui/switch/switch.svelte";
    import { onMount } from "svelte";
    import { emit, listen } from "@tauri-apps/api/event";
    import Button from "$lib/components/ui/button/button.svelte";
    import { Reload } from "svelte-radix";
    import type { programs_payload } from "src/types/programs_payload";
    import type { apps } from "src/types/apps";
    import { CloudFog } from "lucide-svelte";
    let programs: apps[] = [];

    onMount(async () => {
        console.log("robie");
        if (Object.keys(programs).length == 0) {
            emit("programs_request");
        }

        await listen("programs_send", (e: programs_payload) => {
            programs = e.payload.programs;
            console.log("pobrano ponownie");
            emit("stopSending");
        });
    });
    function HideApp(index: number) {
        programs[index].visible = false;
        emit("programs-visibility-changed", { programs: programs });
    }
    function ShowApp(index: number) {
        programs[index].visible = true;
        emit("programs-visibility-changed", { programs: programs });
    }
    function ChangeAppVisibility(index: number) {
        if (programs[index].visible == true) {
            HideApp(index);
        } else {
            ShowApp(index);
        }
    }
</script>

<div class="p-3 space-y-2 !overflow-auto h-full">
    <Label class="text-gray-400">Apps</Label>
    {#if Object.keys(programs).length > 0}
        <div>
            {#each programs as app, index}
                <div class="flex w-full gap-4 items-center my-3">
                    <!-- svelte-ignore a11y-missing-attribute -->
                    <div
                        class:opacity-50={!app.visible}
                        class="flex items-center gap-2"
                    >
                        <img
                            class="h-6 w-6 mr-1"
                            src={`app_icons/${app.name}.png`}
                        />
                        <Label>{app.name}</Label>
                    </div>
                    <Switch
                        checked={programs[index].visible}
                        on:click={() => ChangeAppVisibility(index)}
                    />
                </div>
            {/each}
        </div>
    {:else}
        <div>
            <Button variant={"ghost"}>
                <Reload class="mr-2 h-4 w-4 animate-spin" />
                Loading...
            </Button>
        </div>
    {/if}
</div>
