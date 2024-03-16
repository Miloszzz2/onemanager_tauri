export function fixBackslashes(path: string) {

    return path.replace(/\\\\/g, "\\").replace(/^\\+|\\+$/g, "");
}
