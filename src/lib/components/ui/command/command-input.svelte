<script lang="ts">
    import { cn } from "$lib/utils";
    import { invoke } from "@tauri-apps/api";
    import { Command as CommandPrimitive } from "cmdk-sv";
    import { Globe } from "lucide-svelte";
    import { onMount } from "svelte";
    import MagnifyingGlass from "svelte-radix/MagnifyingGlass.svelte";
    import { toast } from "svelte-sonner";
    type $$Props = CommandPrimitive.InputProps;
    export let mode: string;
    let className: string | undefined | null = undefined;
    export { className as class };
    export let value: string = "";
    let url: string;
    const regex = /^([a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}(\.[a-zA-Z]{2,})?$/;
    function isValidHttpUrl(url: string) {
        try {
            const newUrl = new URL(url);
            return newUrl.protocol === "http:" || newUrl.protocol === "https:";
        } catch (err) {
            return false;
        }
    }
    onMount(() => {
        function handleKeydown(e: KeyboardEvent) {
            console.log(e.key);
            if (value === "" && e.key == "/" && mode != "browser") {
                e.preventDefault();
                mode = "browser";
            } else if (
                e.key === "Backspace" &&
                value == "" &&
                mode == "browser"
            ) {
                mode = "search";               
            }
            if (e.key == "Enter" && mode == "browser") {
                if (!isValidHttpUrl(value) && !regex.test(value))
                    url = "https://google.com/search?q=" + value;
                else if (!isValidHttpUrl(value) && regex.test(value)) {
                    url = "https://" + value;
                } else if (isValidHttpUrl(value)) url = value;
                invoke("search", {
                    url: url,
                });
            }
        }

        document.addEventListener("keydown", handleKeydown);

        return () => {
            document.removeEventListener("keydown", handleKeydown);
        };
    });
</script>

<div class="flex items-center border-b px-3" data-cmdk-input-wrapper="">
    {#if mode == "browser"}
        <Globe class="mr-2 h-4 w-4 shrink-0 opacity-50" />
    {:else}
        <MagnifyingGlass class="mr-2 h-4 w-4 shrink-0 opacity-50" />
    {/if}

    <CommandPrimitive.Input
        class={cn(
            "flex h-10 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50",
            className,
        )}
        {...$$restProps}
        bind:value
        {mode}
    />
</div>
