<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import { cn } from "$lib/utils.js";
    import * as Tooltip from "$lib/components/ui/tooltip";
    import type { Route } from "./config.js";
    import { Link } from "svelte-routing";
    export let isCollapsed: boolean;
    export let routes: Route[];
    let activeRoute: number = 0;
    const changeActiveRoute = (route: Route, index: number) => {
        routes[activeRoute].variant = "ghost";
        route.variant = "default";
        activeRoute = index;
    };
</script>

<div
    data-collapsed={isCollapsed}
    class="group flex flex-col gap-4 py-2 data-[collapsed=true]:py-2"
>
    <nav
        class="grid gap-1 px-2 group-[[data-collapsed=true]]:justify-center group-[[data-collapsed=true]]:px-2"
    >
        {#each routes as route, index}
            {#if isCollapsed}
                <Tooltip.Root openDelay={0}
                    ><Link
                        to={route.href}
                        on:click={() => changeActiveRoute(route, index)}
                    >
                        <Tooltip.Trigger asChild let:builder>
                            <Button
                                builders={[builder]}
                                variant={route.variant}
                                size="icon"
                                class={cn(
                                    "size-9",
                                    route.variant === "default" &&
                                        "dark:bg-muted dark:text-muted-foreground dark:hover:bg-muted dark:hover:text-white",
                                )}
                            >
                                <svelte:component
                                    this={route.icon}
                                    class="size-4"
                                    aria-hidden="true"
                                />
                                <span class="sr-only">{route.title}</span>
                            </Button>
                        </Tooltip.Trigger>
                        <Tooltip.Content
                            side="right"
                            class="flex items-center gap-4"
                        >
                            {route.title}
                            {#if route.label}
                                <span class="ml-auto text-muted-foreground">
                                    {route.label}
                                </span>
                            {/if}
                        </Tooltip.Content>
                    </Link>
                </Tooltip.Root>
            {:else}
                <Link
                    to={route.href}
                    class="w-full"
                    on:click={() => changeActiveRoute(route, index)}
                >
                    <Button
                        variant={route.variant}
                        size="sm"
                        class={cn("w-full justify-start", {
                            "dark:bg-muted dark:text-white dark:hover:bg-muted dark:hover:text-white":
                                route.variant === "default",
                        })}
                    >
                        <svelte:component
                            this={route.icon}
                            class="mr-2 size-4"
                            aria-hidden="true"
                        />
                        {route.title}
                    </Button>
                </Link>
            {/if}
        {/each}
    </nav>
</div>
