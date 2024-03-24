import * as Icons from "./icons.js";

import type { ComponentType } from "svelte";
import type { Icon } from "lucide-svelte";
import { Archive } from "svelte-radix";

export type Route = {
    title: string;
    label: string;
    href: string;
    icon: ComponentType<Icon>;
    variant: "default" | "ghost";
};

export const primaryRoutes: Route[] = [
    {
        title: "Overview",
        href: "/",
        label: "",
        icon: Icons.Settings,
        variant: "default",
    },

    {
        title: "Apps",
        href: "/apps",
        label: "",
        icon: Icons.LayoutGrid,
        variant: "ghost",
    },
    {
        title: "Integrations",
        label: "",
        href: "/integrations",
        icon: Icons.Blocks,
        variant: "ghost",
    },
    {
        title: "Profile",
        href: "/profile",
        label: "",
        icon: Icons.CircleUser,
        variant: "ghost",
    },
    {
        title: "Preferences",
        label: "",
        href: "/preferences",
        icon: Icons.Settings2,
        variant: "ghost",
    },
    {
        title: "Help",
        label: "",
        href: "/help",
        icon: Icons.CircleHelp,
        variant: "ghost",
    },
];
