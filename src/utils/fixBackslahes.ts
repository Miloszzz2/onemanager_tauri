export function fixBackslashes(arr: any) {
    let res: string[] = [];
    arr.map((el: string) => {
        res.push(el.replace(/\\\\/g, "\\").replace(/^\\+|\\+$/g, ""));
    });
    return res;
}
