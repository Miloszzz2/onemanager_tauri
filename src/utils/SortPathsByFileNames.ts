import type { apps } from "src/types/apps";

export function SortPathsByFileNames(paths: apps[]) {
    paths.sort((a, b) => {
        const nameA = a.name; // Convert names to uppercase for case-insensitive comparison
        const nameB = b.name;
        if (nameA < nameB) {
            return -1; // a should come before b in the sorted order
        }
        if (nameA > nameB) {
            return 1; // a should come after b in the sorted order
        }
        return 0; // a and b are equivalent
    });
    return paths;
}