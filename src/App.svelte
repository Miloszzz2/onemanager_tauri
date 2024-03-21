<script lang="ts">
    import { ModeWatcher } from "mode-watcher";
    import QuestionMark from "svelte-radix/QuestionMark.svelte";
    import * as Command from "$lib/components/ui/command";
    import { appWindow } from "@tauri-apps/api/window";
    import { unregister, register } from "@tauri-apps/api/globalShortcut";
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { WebviewWindow } from "@tauri-apps/api/window";
    import { Delay } from "./utils/Delay";
    import { CreateSettingsWindow } from "./utils/CreateSettingsWindow";
    import { emit, listen } from "@tauri-apps/api/event";
    import Button from "$lib/components/ui/button/button.svelte";
    import Reload from "svelte-radix/Reload.svelte";
    import { SortPathsByFileNames } from "./utils/SortPathsByFileNames";
    import { Toaster } from "$lib/components/ui/sonner";
    import type { programs_payload } from "./types/programs_payload";
    import type { apps } from "./types/apps";
    import Music from "$lib/components/main/Music.svelte";
    /* Imports here */

    export let open: boolean = true;

    let value: string = "";
    let programs: apps[] = [];

    function getProgramPaths() {
        invoke("getprogrampaths").then((message: apps[] | any) => {
            programs = SortPathsByFileNames(message);
            console.log(Object.keys(programs).length);
        });
    }

    window.addEventListener("DOMContentLoaded", async () => {
        await unregister("Enter");
        await unregister("Control+Shift+K");
        await register("Control+Shift+K", async () => {
            console.log(open);
            if (open == true) {
                CloseMenu();
            } else {
                OpenMenu();
            }
        });
        await unregister("Escape");
        await register("Escape", async () => {
            CloseMenu();
        });
        console.log("registered keyboard shortcut");
    });
    async function OpenMenu() {
        await appWindow.unminimize();
        appWindow.setFocus();
        await Delay(500);
        open = true;
    }
    async function CloseMenu() {
        open = false;
        await Delay(201);
        appWindow.minimize();
    }
    onMount(() => {
        if (programs && Object.keys(programs).length == 0) {
            console.log("pobieram programy ponownie");
            getProgramPaths();
        }

        listen("programs_request", () => {
            emit("programs_send", { programs });
        });

        listen("programs-visibility-changed", (e: programs_payload) => {
            programs = e.payload.programs;
        });
    });

    async function OpenSettingsWindow() {
        if (WebviewWindow.getByLabel("Settings") == null)
            CreateSettingsWindow(programs);
        else {
            if (WebviewWindow.getByLabel("Settings")?.isMinimized()) {
                WebviewWindow.getByLabel("Settings")?.unminimize();
                WebviewWindow.getByLabel("Settings")?.setFocus();
            } else {
                WebviewWindow.getByLabel("Settings")?.setFocus();
            }
        }
    }
</script>

{#if programs && Object.keys(programs).length != 0}
    <Command.Dialog bind:open bind:value>
        <Command.Input
            placeholder="Type a command or search...."
            on:input={() => console.log("input")}
        />
        <Command.List>
            <Command.Empty>No results found.</Command.Empty>
            <Music />
            <Command.Group heading="Apps">
                {#each programs as app}
                    {#if app.visible}
                        <!-- svelte-ignore a11y-no-static-element-interactions -->
                        <!-- svelte-ignore a11y-click-events-have-key-events -->
                        <div
                            class="overflow-y-hidden"
                            on:click={() => {
                                invoke("run_program", {
                                    path: app.path,
                                });
                                CloseMenu();
                            }}
                        >
                            <Command.Item>
                                <!-- svelte-ignore a11y-missing-attribute -->
                                <img
                                    class="h-6 w-6 mr-3"
                                    src={`app_icons/${app.name}.png`}
                                />
                                <span>{app.name}</span>
                            </Command.Item>
                        </div>
                    {/if}
                {/each}
            </Command.Group>
            <Command.Separator />
        </Command.List>
        <Button
            class="absolute bottom-4 right-6 rounded-full z-40 h-10 w-10 p-2 border cursor-pointer"
            on:click={OpenSettingsWindow}
        >
            <QuestionMark class="dark:white light:black" />
        </Button>
    </Command.Dialog>
{:else}
    <div
        class="h-full flex items-center justify-center rounded-lg bg-background"
        style="height:349px;"
    >
        <Button variant={"ghost"} size={"lg"}>
            <Reload class="mr-2 h-4 w-4 animate-spin" />
            Loading...
        </Button>
    </div>
{/if}
<Toaster />
<ModeWatcher />
