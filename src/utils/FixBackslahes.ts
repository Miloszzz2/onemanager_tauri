export function FixBackslashes(path: string) {

    return path.replace(/\\\\/g, "\\").replace(/^\\+|\\+$/g, "");
}
