<script lang="ts">
    import { ModeWatcher } from "mode-watcher";
    import QuestionMark from "svelte-radix/QuestionMark.svelte";
    import * as Command from "$lib/components/ui/command";
    import { appWindow } from "@tauri-apps/api/window";
    import {
        unregister,
        register,
        isRegistered,
    } from "@tauri-apps/api/globalShortcut";
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { WebviewWindow } from "@tauri-apps/api/window";
    import { fixBackslashes } from "./utils/fixBackslahes";
    import { delay } from "./utils/delay";
    import { createSettingsWindow } from "./utils/createSettingsWindow";
    import { emit, listen } from "@tauri-apps/api/event";
    import Button from "$lib/components/ui/button/button.svelte";
    import Reload from "svelte-radix/Reload.svelte";
    import { SortPathsByFileNames } from "./utils/sortPathsByFileNames";
    import { Toaster } from "$lib/components/ui/sonner";
    import type { programs_payload } from "./types/programs_payload";
    import type { apps } from "./types/apps";
    /* Imports here */

    export let open: boolean = true;

    let value: string = "";
    let programs: apps[] = [];

    function getProgramPaths() {
        invoke("getprogrampaths").then((message: apps[] | any) => {
            programs = SortPathsByFileNames(message);
        });
    }

    window.addEventListener("DOMContentLoaded", async () => {
        await unregister("Control+Shift+K");
        await register("Control+Shift+K", async () => {
            console.log(open);
            if (open == true) {
                closeMenu();
            } else {
                openMenu();
            }
        });
        await unregister("Escape");
        await register("Escape", async () => {
            closeMenu();
        });
        console.log("registered keyboard shortcut");
    });
    export async function openMenu() {
        await appWindow.unminimize();
        appWindow.setFocus();
        await delay(500);
        open = true;
    }
    export async function closeMenu() {
        open = false;
        await delay(201);
        appWindow.minimize();
    }
    onMount(() => {
        async function handleKeydown(e: KeyboardEvent) {
            if (e.key === "Enter") {
                invoke("run_program", { path: value });
                closeMenu();
            }
        }
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

        document.addEventListener("keydown", handleKeydown);
        return () => {
            document.removeEventListener("keydown", handleKeydown);
        };
    });

    async function openSettingsWindow() {
        if (WebviewWindow.getByLabel("Settings") == null)
            createSettingsWindow(programs);
        else {
            if (WebviewWindow.getByLabel("Settings")?.isMinimized()) {
                WebviewWindow.getByLabel("Settings")?.unminimize();
                WebviewWindow.getByLabel("Settings")?.setFocus();
            } else {
                WebviewWindow.getByLabel("Settings")?.setFocus();
            }
        }
    }
    let inputValue = "";
</script>

{#if programs && Object.keys(programs).length != 0}
    <Command.Dialog bind:open bind:value>
        <Command.Input
            bind:value={inputValue}
            placeholder="Type a command or search...."
            on:input={() => console.log("input")}
        />
        <Command.List>
            <Command.Empty>No results found.</Command.Empty>
            <Command.Group heading="Apps">
                {#each programs as app}
                    {#if app.visible}
                        <!-- svelte-ignore a11y-no-static-element-interactions -->
                        <!-- svelte-ignore a11y-click-events-have-key-events -->
                        <div
                            class="overflow-y-hidden"
                            on:click={() => {
                                invoke("run_program", {
                                    path: fixBackslashes(app.path),
                                });
                                closeMenu();
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
            on:click={openSettingsWindow}
        >
            <QuestionMark class="dark:white light:black" />
        </Button>
    </Command.Dialog>
{:else}
    <div
        class="h-full flex items-center justify-center rounded-lg bg-background"
        style="height:349px;"
    >
        <Button variant={"ghost"}>
            <Reload class="mr-2 h-4 w-4 animate-spin" />
            Loading...
        </Button>
    </div>
{/if}
<Toaster />
<ModeWatcher />
