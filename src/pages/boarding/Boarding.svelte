<script lang="ts">
    import { ModeWatcher } from "mode-watcher";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as Carousel from "$lib/components/ui/carousel/index.js";
    import X from "lucide-svelte/icons/x";
    import Minus from "lucide-svelte/icons/minus";
    import { appWindow } from "@tauri-apps/api/window";
    import { type CarouselAPI } from "$lib/components/ui/carousel/context.js";
    import Button from "$lib/components/ui/button/button.svelte";
    import { WebviewWindow } from "@tauri-apps/api/window";
    import Welcome from "../components/boarding/Welcome.svelte";
    import Browser from "../components/boarding/Browser.svelte";
    import Login from "../components/boarding/Login.svelte";
    import Favourites from "../components/boarding/Favourites.svelte";
    import { emit } from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api";
    import { SortPathsByFileNames } from "$utils/SortPathsByFileNames";
    import type { apps } from "src/types/apps";
    import { db_pool } from "$db/db";
    import type Database from "tauri-plugin-sql-api";

    let api: CarouselAPI;
    let current = 0;
    let count = 4;
    let programs: apps[] = [];
    function getProgramPaths() {
        invoke("getprogrampaths").then((message: apps[] | any) => {
            programs = SortPathsByFileNames(message as apps[]);
            console.log(programs);
            console.log(Object.keys(programs).length);
        });
    }

    onMount(async () => {
        const db: Database = await db_pool();
        if (programs && Object.keys(programs).length == 0) {
            console.log("pobieram programy ponownie");
            getProgramPaths();
        }
    });
</script>

{#if Object.keys(programs).length > 0}
    <div class="flex justify-between w-full sticky top-0 left-0 z-10 border-b">
        <div
            class="flex gap-3 py-3 pt-3 pl-3 items-center hover:opacity-70 transition-all
            duration-300 cursor-pointer"
        >
            <p class="pl-1 text-md font-medium">OneManager</p>
        </div>
        <div
            data-tauri-drag-region
            class="titlebar w-full flex justify-end p-3 gap-2"
        >
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div
                on:click={() => {
                    appWindow.minimize();
                }}
            >
                <Minus class="cursor-pointer" size={22} />
            </div>
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div
                on:click={() => {
                    appWindow.hide();
                }}
            >
                <X class="cursor-pointer" size={22} />
            </div>
        </div>
    </div>
    <Carousel.Root class="w-full" bind:api>
        <Carousel.Content class="h-[550px] ">
            <Welcome />
            <Browser />
            <Login />
            <Favourites {programs} />
        </Carousel.Content>
        <div class="flex gap-3 justify-end px-4 py-2 absolute bottom-3 right-2">
            <Button variant={"outline"} on:click={() => api.scrollPrev()}
                >Previous</Button
            >
            <Button
                on:click={() => {
                    current++;
                    if (current == count) {
                        appWindow.hide();
                        WebviewWindow.getByLabel("main")?.show();
                        emit("user_programs", { programs: programs });
                    } else {
                        api.scrollNext();
                    }
                }}>Next</Button
            >
        </div>
    </Carousel.Root>
    <ModeWatcher />
{/if}
