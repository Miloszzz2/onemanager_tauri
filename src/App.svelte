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
    import { emit, listen } from "@tauri-apps/api/event";
    import Button from "$lib/components/ui/button/button.svelte";
    import Reload from "svelte-radix/Reload.svelte";
    import { SortPathsByFileNames } from "./utils/SortPathsByFileNames";
    import { Toaster } from "$lib/components/ui/sonner";
    import type { programs_payload } from "./types/programs_payload";
    import type { apps } from "./types/apps";
    import { db_pool } from "$db/db";
    import type Database from "tauri-plugin-sql-api";
    import { Globe } from "lucide-svelte";
    /* Imports here */

    export let open: boolean = true;

    let value: string = "";
    let programs: apps[] = [];
    $: mode = "search";
    function getProgramPaths() {
        invoke("getprogrampaths").then((message: apps[] | any) => {
            programs = SortPathsByFileNames(message as apps[]);
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
    onMount(async () => {
        const db: Database = await db_pool();
        listen("user_programs", (e: programs_payload) => {
            programs = e.payload.programs;
        });
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
        await WebviewWindow.getByLabel("Settings")?.show();
    }
</script>

{#if programs && Object.keys(programs).length != 0}
    <Command.Dialog bind:open bind:value>
        <Command.Input
            placeholder="Type a command or search...."
            on:change={() => console.log("Hello world")}
            bind:mode
        />
        <Command.List>
            {#if mode == "search"}
                <Command.Empty>No results found.</Command.Empty>
                <Command.Group heading="Favourites">
                    {#each programs as app}
                        {#if app.favourite}
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
                <Command.Group heading="All Apps">
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
            {:else}
                <div class="flex justify-center h-[80px] items-center gap-2">
                    <p class="text-muted-foreground font-medium">
                        Browser mode
                    </p>
                </div>
            {/if}
        </Command.List>
        <Button
            class="absolute bottom-4 right-6 rounded-full z-40 h-10 w-10 p-2 border cursor-pointer"
            on:click={OpenSettingsWindow}
        >
            <QuestionMark class="dark:white light:black" />
        </Button>
        <Toaster />
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

<ModeWatcher />
