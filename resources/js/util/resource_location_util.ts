export function validateResourceLocation(resourceLocation: string): boolean {
    let regex = /[a-z0-9\._-]+:[a-z0-9\._-]+/
    return regex.test(resourceLocation)
}