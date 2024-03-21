<script lang="ts">
    import type { integrations } from "src/types/integrations";
    import Button from "../ui/button/button.svelte";
    import * as Card from "../ui/card";
    import Label from "../ui/label/label.svelte";
    const integrations: integrations[] = [
        {
            name: "Google Calendar",
            description:
                "Connect Google Calendar to simply manage your calendar",
            image: "integrations/google-calendar.png",
        },
        {
            name: "Notion",
            description: "Manage databases and macros",
            image: "integrations/notion.png",
        },
        {
            name: "Spotify",
            description: "Access your playlist and song queue from onemanager",
            image: "integrations/spotify.png",
        },
        {
            name: "Notion Calendar",
            description: "Create events in Notion Calendar",
            image: "integrations/notion-calendar.png",
        },
        {
            name: "Github",
            description: "Automate pull requests, manage your repositories",
            image: "integrations/github.png",
        },
        {
            name: "Linear",
            description: "Manage your linear projects",
            image: "integrations/linear.png",
        },
    ];
</script>

<div class="p-3 !overflow-y-auto h-full">
    <Label class="text-gray-400">Integrations</Label>
    <div class="grid grid-cols-3 gap-2 my-3">
        {#each integrations as integration}
            <Card.Root class="hover:bg-border">
                <Card.Header class="min-h-[170px] flex-col gap-3">
                    <img src={integration.image} alt="" class="h-12 w-12" />
                    <div>
                        <Card.Title>{integration.name}</Card.Title>
                        <Card.Description class="mt-2"
                            >{integration.description}</Card.Description
                        >
                    </div>
                </Card.Header>
                <Card.Footer class="flex justify-end">
                    <Button>Integrate</Button>
                </Card.Footer>
            </Card.Root>
        {/each}
    </div>
</div>
