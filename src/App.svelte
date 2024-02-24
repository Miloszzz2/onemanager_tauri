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
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  let open = true;
  let inputvalue = "";
  let value = "";
  // Function to delay execution
  function delay(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }
  function fixBackslashes(arr: any) {
    let res: String[] = [];
    arr.map((el: String) => {
      res.push(el.replace(/\\\\/g, "\\"));
    });
    return res;
  }
  let programs: String[] = [];
  appWindow.listen("created", async () => {
    invoke("getprogrampaths").then(
      (message) => (programs = fixBackslashes(message))
    );
    appWindow.setFocus();
    await register("Control+Shift+K", async () => {
      if (open === true) {
        inputvalue = "";
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
  async function closeMenu() {
    inputvalue = "";
    open = !open;
    await delay(500);
    appWindow.minimize();
  }

  appWindow.emit("created");
  onMount(() => {
    function handleKeydown(e: KeyboardEvent) {
      if (e.key === "Enter") {
        invoke("runprogram", { path: value });
        closeMenu();
      } else if (e.key === "Delete") {
        let indextodel = 0;
        programs.forEach((el, index) => {
          if (el.toLowerCase() === value.toLowerCase()) indextodel = index;
        });
        programs.splice(indextodel, 1);

        programs = programs;
        console.log(programs.length);
      }
    }

    document.addEventListener("keydown", handleKeydown);
    return () => {
      document.removeEventListener("keydown", handleKeydown);
    };
  });
</script>

<Command.Dialog bind:open bind:value>
  <Command.Input placeholder="Type a command or search..." />
  <Command.List>
    <Command.Empty>No results found.</Command.Empty>
    <Command.Group heading="Apps">
      {#each programs as app}
        <Command.Item>
          <!-- svelte-ignore a11y-no-static-element-interactions -->
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <div
            on:click={() => {
              invoke("runprogram", { path: app });
              closeMenu();
            }}
          >
            <span>{app}</span>
          </div>
        </Command.Item>
      {/each}
    </Command.Group>
    <Command.Separator />
  </Command.List>
</Command.Dialog>
<ModeWatcher />
