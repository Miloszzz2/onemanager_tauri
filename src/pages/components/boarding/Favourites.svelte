<script lang="ts">
    import * as Carousel from "$lib/components/ui/carousel/index.js";
    import * as Card from "$lib/components/ui/card/index.js";
    import type { apps } from "src/types/apps";
    import Label from "$lib/components/ui/label/label.svelte";
    import { Star } from "lucide-svelte";
    import type Database from "tauri-plugin-sql-api";
    export let programs: apps[];
    function ChangeAppFavouriteValue(index: number) {
        programs[index].favourite = !programs[index].favourite;
    }
</script>

<Carousel.Item class="!overflow-y-scroll">
    <div class="">
        <Card.Root class="border-none ">
            <Card.Content class="p-0 pl-6 flex flex-col  w-full "
                ><Label class="text-gray-400 text-left pt-6 pb-2"
                    >Select your most-used apps, which will be displayed on top
                    in OneManager</Label
                >
                <div class="w-full">
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
                            <!-- svelte-ignore a11y-click-events-have-key-events -->
                            <!-- svelte-ignore a11y-no-static-element-interactions -->
                            <div
                                on:click={() => ChangeAppFavouriteValue(index)}
                                class="cursor-pointer"
                            >
                                <Star fill={app.favourite ? "white" : ""} />
                            </div>
                        </div>
                    {/each}
                </div>
            </Card.Content>
        </Card.Root>
    </div>
</Carousel.Item>
