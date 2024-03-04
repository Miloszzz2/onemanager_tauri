export function getAppNameFromPath(app: string) {
    let lastIndex = app.lastIndexOf("\\");

    return (
        app
            .substring(lastIndex + 1, app.length - 4)
            .charAt(0)
            .toUpperCase() + app.substring(lastIndex + 2, app.length - 4)
    );
}
