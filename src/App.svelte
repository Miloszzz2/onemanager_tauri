<script lang="ts">
  import Calendar from "svelte-radix/Calendar.svelte";
  import EnvelopeClosed from "svelte-radix/EnvelopeClosed.svelte";
  import Face from "svelte-radix/Face.svelte";
  import Gear from "svelte-radix/Gear.svelte";
  import Person from "svelte-radix/Person.svelte";
  import Rocket from "svelte-radix/Rocket.svelte";
  import { ModeWatcher } from "mode-watcher";
  import * as Command from "$lib/components/ui/command";
  import { appWindow } from "@tauri-apps/api/window";
  import { register } from "@tauri-apps/api/globalShortcut";
  let open = true;
  let value = "";
  // Function to delay execution
  function delay(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }
  appWindow.listen("created", async () => {
    appWindow.setFocus();
    await register("Control+Shift+K", async () => {
      if (open === true) {
        value = "";
        open = !open;
        await delay(500);
        appWindow.minimize();
      } else {
        await appWindow.unminimize();
        appWindow.setFocus();
        await delay(500);
        open = !open;
      }
    });
  });
  appWindow.emit("created");
</script>

<Command.Dialog bind:open>
  <Command.Input placeholder="Type a command or search..." bind:value />
  <Command.List>
    <Command.Empty>No results found.</Command.Empty>
    <Command.Group heading="Suggestions">
      <Command.Item>
        <Calendar class="mr-2 h-4 w-4" />
        <span>Calendar</span>
      </Command.Item>
      <Command.Item>
        <Face class="mr-2 h-4 w-4" />
        <span>Search Emoji</span>
      </Command.Item>
      <Command.Item>
        <Rocket class="mr-2 h-4 w-4" />
        <span>Launch</span>
      </Command.Item>
    </Command.Group>
    <Command.Separator />
    <Command.Group heading="Settings">
      <Command.Item>
        <Person class="mr-2 h-4 w-4" />
        <span>Profile</span>
        <Command.Shortcut>⌘P</Command.Shortcut>
      </Command.Item>
      <Command.Item>
        <EnvelopeClosed class="mr-2 h-4 w-4" />
        <span>Mail</span>
        <Command.Shortcut>⌘B</Command.Shortcut>
      </Command.Item>
      <Command.Item>
        <Gear class="mr-2 h-4 w-4" />
        <span>Settings</span>
        <Command.Shortcut>⌘S</Command.Shortcut>
      </Command.Item>
    </Command.Group>
  </Command.List>
</Command.Dialog>
<ModeWatcher />
