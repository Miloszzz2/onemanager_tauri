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
    import { getAuth, onAuthStateChanged, type User } from "firebase/auth";
    const auth = getAuth();
    let current_user: User | null = null;
    onAuthStateChanged(auth, (user) => {
        current_user = user;
        if (
            user &&
            localStorage.getItem("configuration_completed") ==
                JSON.stringify(true)
        ) {
            appWindow.hide();
            WebviewWindow.getByLabel("main")?.show();
            emit("user_programs", { programs: programs });
        }
    });

    let api: CarouselAPI;
    let current = 0;
    let count = 0;
    let programs: apps[] = [];
    let configuration_completed = localStorage.getItem(
        "configuration_completed",
    );
    $: if (api) {
        count = api.scrollSnapList().length;
        current = api.selectedScrollSnap() + 1;
        api.on("select", () => {
            current = api.selectedScrollSnap() + 1;
        });
        console.log(count);
    }
    function getProgramPaths() {
        invoke("getprogrampaths").then((message: apps[] | any) => {
            programs = SortPathsByFileNames(message as apps[]);
            console.log(programs);
            console.log(Object.keys(programs).length);
        });
    }

    onMount(async () => {
        configuration_completed = JSON.stringify(
            localStorage.getItem("configuration_completed"),
        );
        console.log(configuration_completed);
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
            {#if configuration_completed == "null"}
                <Welcome />
            {:else if configuration_completed == "true"}{/if}
            <Login />
            {#if configuration_completed == "null"}
                <Browser />
                <Favourites {programs} />
            {:else if configuration_completed == "true"}{/if}
        </Carousel.Content>
        {#if count > 1}
            <div
                class="flex gap-3 justify-end px-4 py-2 absolute bottom-3 right-2"
            >
                <Button
                    variant={"outline"}
                    on:click={() => {
                        api.scrollPrev();
                    }}>Previous</Button
                >

                {#if count > 1 && (current != 2 || current_user)}
                    <Button
                        on:click={() => {
                            if (current == count && count > 1) {
                                appWindow.hide();

                                localStorage.setItem(
                                    "configuration_completed",
                                    JSON.stringify(true),
                                );

                                WebviewWindow.getByLabel("main")?.show();
                                emit("user_programs", { programs: programs });
                                location.reload();
                            } else {
                                api.scrollNext();
                            }
                        }}>Next</Button
                    >
                {/if}
            </div>
        {/if}
    </Carousel.Root>
    <ModeWatcher />
{/if}
