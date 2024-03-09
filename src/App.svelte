<script lang="ts">
    import { ModeWatcher } from "mode-watcher";
    import QuestionMark from "svelte-radix/QuestionMark.svelte";
    import * as Command from "$lib/components/ui/command";
    import { appWindow } from "@tauri-apps/api/window";
    import { unregister, register } from "@tauri-apps/api/globalShortcut";
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { WebviewWindow } from "@tauri-apps/api/window";
    import { fixBackslashes } from "./utils/fixBackslahes";
    import { getAppNameFromPath } from "./utils/getAppNameFromPath";
    import { delay } from "./utils/delay";
    import { createSettingsWindow } from "./utils/createSettingsWindow";
    import { emit, listen } from "@tauri-apps/api/event";
    /* Imports here */

    let open = true;
    let inputvalue = "";
    let value = "";
    let programs: string[] = [];

    window.addEventListener("load", async () => {
        invoke("getprogrampaths").then((message) => {
            programs = fixBackslashes(message);
        });
        appWindow.setFocus();

        await unregister("Control+Shift+K");
        await register("Control+Shift+K", async () => {
            if (open == true) {
                closeMenu();
            } else {
                await appWindow.unminimize();
                appWindow.setFocus();
                await delay(500);
                open = true;
            }
            console.log(open);
        });

        console.log("registered");
    });

    async function closeMenu() {
        inputvalue = "";
        open = false;
        await delay(201);
        appWindow.minimize();
    }
    const sendProgramPathsInterval = setInterval(() => {
        console.log("sending paths...");
        emit("click", {
            programs,
        });
    }, 2000);
    onMount(() => {
        async function handleKeydown(e: KeyboardEvent) {
            if (e.key === "Enter") {
                invoke("runprogram", { path: value });
                closeMenu();
            } else if (e.key === "Delete") {
                let indextodel = 0;
                programs.forEach((el, index) => {
                    if (el.toLowerCase() === value.toLowerCase())
                        indextodel = index;
                });
                programs.splice(indextodel, 1);
                programs = programs;
                console.log(programs.length);
            } else if (e.key === "Escape") {
                closeMenu();
            }
        }

        listen("stopSending", () => {
            clearInterval(sendProgramPathsInterval);
            console.log("stopped sending");
        });

        console.log(programs.length);
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
    console.log(appWindow.label);
</script>

{#if programs.length > 0}
    <Command.Dialog bind:open bind:value>
        <Command.Input placeholder="Type a command or search..." />
        <Command.List>
            <Command.Empty>No results found.</Command.Empty>
            <Command.Group heading="Apps">
                {#each programs as app}
                    <!-- svelte-ignore a11y-no-static-element-interactions -->
                    <!-- svelte-ignore a11y-click-events-have-key-events -->
                    <div
                        class="overflow-y-hidden"
                        on:click={() => {
                            invoke("runprogram", { path: app });
                            closeMenu();
                        }}
                    >
                        <Command.Item>
                            <img
                                class="h-6 w-6 mr-3"
                                src={`app_icons/${getAppNameFromPath(app)}.png`}
                                alt="app_icons/HWMonitor.png"
                            />
                            <span>{getAppNameFromPath(app)}</span>
                        </Command.Item>
                    </div>
                {/each}
            </Command.Group>
            <Command.Separator />
        </Command.List>
        <QuestionMark
            on:click={openSettingsWindow}
            color="black"
            class="absolute bottom-4 right-6 bg-slate-50 rounded-full z-40 h-10 w-10 p-2 border cursor-pointer"
        />
    </Command.Dialog>
    <ModeWatcher />
{/if}
