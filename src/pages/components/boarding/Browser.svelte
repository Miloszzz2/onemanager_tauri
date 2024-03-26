<script lang="ts">
    import * as Carousel from "$lib/components/ui/carousel/index.js";
    import * as Card from "$lib/components/ui/card/index.js";
    import Label from "$lib/components/ui/label/label.svelte";
    import type { browser } from "src/types/browsers";
    let active: number;
    let browsers: browser[] = [
        { name: "Chrome", image: "browsers/chrome.png" },
        { name: "Firefox", image: "browsers/firefox.png" },
        { name: "Brave", image: "browsers/brave.png" },
        { name: "Edge", image: "browsers/edge.png" },
        { name: "Opera", image: "browsers/opera.png" },
        { name: "Vivaldi", image: "browsers/vivaldi.png" },
    ];
</script>

<Carousel.Item class="h-full">
    <div class="">
        <Card.Root class="border-none">
            <Card.Content class=" flex flex-col w-full h-[490px] ">
                <Label class="text-gray-400 text-left pt-6 pb-2"
                    >Select your default browser app. It will be used as your
                    default primary brwoser. You can change it whenever you want
                    in settings</Label
                >
                <div class="grid grid-cols-3 gap-3 w-full mt-5">
                    {#each browsers as browser, index}
                        <Card.Root
                            class={`${active == index ? "border-white " : ""} border-2 cursor-pointer flex flex-col items-center justify-center`}
                            on:click={() => {
                                active = index;
                                console.log(active);
                            }}
                        >
                            <Card.Content class="pt-4"
                                ><img
                                    class="h-[120px]"
                                    src={browser.image}
                                    alt={browser.name}
                                /></Card.Content
                            >
                            <Card.Footer class="text-center ">
                                <Label class="text-gray-400 "
                                    >{browser.name}</Label
                                >
                            </Card.Footer>
                        </Card.Root>
                    {/each}
                </div>
            </Card.Content>
        </Card.Root>
    </div>
</Carousel.Item>
