<script lang="ts">
    import * as Carousel from "$lib/components/ui/carousel/index.js";
    import * as Card from "$lib/components/ui/card/index.js";
    import { GithubLogo } from "svelte-radix";
    import Button from "$lib/components/ui/button/button.svelte";
    import Input from "$lib/components/ui/input/input.svelte";
    import Label from "$lib/components/ui/label/label.svelte";
    import { cn } from "$lib/utils.js";
    import { LoaderCircle } from "lucide-svelte";
    import { login, logout } from "$services/auth";
    import { getAuth, onAuthStateChanged } from "firebase/auth";
    const auth = getAuth();
    let current_user: any = null;
    onAuthStateChanged(auth, (user) => {
        current_user = user;
    });
    let email = "";

    let className: string | undefined | null = undefined;
    export { className as class };

    let isLoading = false;
</script>

<Carousel.Item class="h-full p-0">
    <div class="">
        {#if current_user == null}
            <Card.Root class="border-none">
                <Card.Content
                    class="p-0 flex items-center justify-center w-full h-[490px] "
                >
                    <div
                        class={cn("grid gap-6 w-72", className)}
                        {...$$restProps}
                    >
                        <h1
                            class="text-center text-2xl font-semibold tracking-tight"
                        >
                            Login
                        </h1>

                        <div class="grid gap-2">
                            <div class="grid gap-1">
                                <Label class="sr-only" for="email">Email</Label>
                                <Input
                                    id="email"
                                    placeholder="name@example.com"
                                    type="email"
                                    autocapitalize="none"
                                    autocomplete="email"
                                    autocorrect="off"
                                    disabled={isLoading}
                                    bind:value={email}
                                />
                            </div>
                            <Button type="submit" disabled={isLoading}>
                                {#if isLoading}
                                    <LoaderCircle
                                        class="mr-2 h-4 w-4 animate-spin"
                                    />
                                {/if}
                                Sign In with Email
                            </Button>
                        </div>

                        <div class="relative">
                            <div class="absolute inset-0 flex items-center">
                                <span class="w-full border-t" />
                            </div>
                            <div
                                class="relative flex justify-center text-xs uppercase"
                            >
                                <span
                                    class="bg-background px-2 text-muted-foreground"
                                >
                                    Or continue with
                                </span>
                            </div>
                        </div>
                        <div class="flex flex-col gap-3">
                            <Button
                                variant="outline"
                                type="button"
                                disabled={isLoading}
                            >
                                {#if isLoading}
                                    <LoaderCircle
                                        class="mr-2 h-4 w-4 animate-spin"
                                    />
                                {:else}
                                    <GithubLogo class="mr-2 h-4 w-4" />
                                {/if}
                                {" "}
                                GitHub
                            </Button>
                            <Button
                                type="button"
                                disabled={isLoading}
                                on:click={() => {
                                    console.log("robie");
                                    login();
                                }}
                            >
                                {#if isLoading}
                                    <LoaderCircle
                                        class="mr-2 h-4 w-4 animate-spin"
                                    />
                                {:else}
                                    <img
                                        src={"icons/google.png"}
                                        class="h-4 pr-2"
                                        alt="google-icon"
                                    />
                                {/if}
                                {" "}
                                Google
                            </Button>
                        </div>
                    </div>
                </Card.Content>
            </Card.Root>
        {:else}
            <Card.Root class="border-none">
                <Card.Content
                    class="flex flex-col gap-5 items-center justify-center w-full h-[490px] "
                >
                    <p class="text-4xl font-semibold">
                        Welcome {current_user.displayName}
                    </p>
                    <p class="text-2xl font-semibold">Successfully Logged In</p>

                    <Button on:click={() => logout()}>Logout</Button>
                </Card.Content>
            </Card.Root>
        {/if}
    </div>
</Carousel.Item>
