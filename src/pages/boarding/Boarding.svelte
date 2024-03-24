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
    let api: CarouselAPI;
    let count = 0;
    let current = 0;
    $: if (api) {
        count = api.scrollSnapList().length;
        current = api.selectedScrollSnap() + 1;
        api.on("select", () => {
            current = api.selectedScrollSnap() + 1;
        });
        if (current == 6) {
            appWindow.hide();
            WebviewWindow.getByLabel("main")?.show();
        }
    }
</script>

<div class="flex justify-between w-full">
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
                appWindow.hide();
            }}
        >
            <X class="cursor-pointer" size={22} />
        </div>
    </div>
</div>
<Carousel.Root class="w-full overflow-hidden" bind:api>
    <Carousel.Content>
        {#each Array(5) as _, i (i)}
            <Carousel.Item class="h-full">
                <div class="">
                    <Card.Root class="border-none">
                        <Card.Content
                            class=" flex items-center justify-center w-full h-[490px] "
                        >
                            <span class="text-4xl font-semibold">{i + 1}</span>
                        </Card.Content>
                    </Card.Root>
                </div>
            </Carousel.Item>
        {/each}
        <Carousel.Item class="h-full">
            <div class="">
                <Card.Root class="border-none">
                    <Card.Content
                        class=" flex items-center justify-center w-full h-[490px] "
                    >
                        <span class="text-4xl font-semibold">6</span>
                    </Card.Content>
                </Card.Root>
            </div>
        </Carousel.Item>
    </Carousel.Content>
    <div class="flex gap-3 justify-end px-4 py-2">
        <Button variant={"outline"} on:click={() => api.scrollPrev()}
            >Previous</Button
        >
        <Button
            on:click={() => {
                console.log(current);
                if (current == 5) {
                    appWindow.hide();
                    WebviewWindow.getByLabel("main")?.show();
                } else {
                    api.scrollNext();
                }
            }}>Next</Button
        >
    </div>
</Carousel.Root>
<ModeWatcher />
