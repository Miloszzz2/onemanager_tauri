<script lang="ts">
    import { ModeWatcher } from "mode-watcher";
    import { onMount } from "svelte";
    import { emit, listen } from "@tauri-apps/api/event";
    import Nav from "$lib/components/settings/nav.svelte";
    import { primaryRoutes } from "$lib/components/settings/config";
    import * as Resizable from "$lib/components/ui/resizable";
    import ChevronLeft from "lucide-svelte/icons/chevron-left";
    import Button from "$lib/components/ui/button/button.svelte";
    import X from "lucide-svelte/icons/x";
    import Minus from "lucide-svelte/icons/minus";
    import { appWindow } from "@tauri-apps/api/window";
    import { PaneGroup } from "paneforge";
    /* Imports here */

    let programs: string[];
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

    export let defaultCollapsed = false;
    export let navCollapsedSize = 6;

    let isCollapsed = defaultCollapsed;

    function onLayoutChange(sizes: number[]) {
        document.cookie = `PaneForge:layout=${JSON.stringify(sizes)}`;
    }

    function onCollapse() {
        isCollapsed = true;
        document.cookie = `PaneForge:collapsed=${true}`;
    }

    function onExpand() {
        isCollapsed = false;
        document.cookie = `PaneForge:collapsed=${false}`;
    }
</script>

<Resizable.PaneGroup
    direction="vertical"
    {onLayoutChange}
    class="items-stretch"
>
    <Resizable.Pane defaultSize={8}>
        <Resizable.PaneGroup direction={"horizontal"} class="border-b">
            <Resizable.Pane defaultSize={10} minSize={10} maxSize={10}>
                <!-- svelte-ignore a11y-no-static-element-interactions -->
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <div
                    class="flex gap-3 items-center py-3 pt-3 pl-3 hover:opacity-70 transition-all
            duration-300 cursor-pointer"
                    on:click={() => {
                        appWindow.close();
                    }}
                >
                    <ChevronLeft size={15} />
                    <p class="text-md font-medium">Settings</p>
                </div>
            </Resizable.Pane>
            <Resizable.Pane defaultSize={90}
                ><div
                    data-tauri-drag-region
                    class="titlebar w-100 flex justify-end p-3 gap-2"
                >
                    <!-- svelte-ignore a11y-no-static-element-interactions -->
                    <!-- svelte-ignore a11y-click-events-have-key-events -->
                    <div
                        on:click={() => {
                            console.log("robie skr");
                            appWindow.minimize();
                        }}
                    >
                        <Minus class="cursor-pointer" size={22} />
                    </div>
                    <!-- svelte-ignore a11y-no-static-element-interactions -->
                    <!-- svelte-ignore a11y-click-events-have-key-events -->
                    <div
                        on:click={() => {
                            appWindow.close();
                        }}
                    >
                        <X class="cursor-pointer" size={22} />
                    </div>
                </div></Resizable.Pane
            >
        </Resizable.PaneGroup>
    </Resizable.Pane>
    <Resizable.Pane defaultSize={92}>
        <Resizable.PaneGroup direction={"horizontal"}>
            <Resizable.Pane
                defaultSize={20}
                collapsedSize={navCollapsedSize}
                collapsible
                minSize={12}
                maxSize={20}
                {onCollapse}
                {onExpand}
            >
                <Nav {isCollapsed} routes={primaryRoutes} />
            </Resizable.Pane>
            <Resizable.Handle />
            <Resizable.Pane defaultSize={80}></Resizable.Pane>
        </Resizable.PaneGroup>
    </Resizable.Pane>
</Resizable.PaneGroup>

<ModeWatcher />
