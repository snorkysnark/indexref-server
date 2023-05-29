export function arrayEq(a: any[], b: any[]): boolean {
    if (a.length !== b.length) return false;

    for (let i = 0; i < a.length; i++) {
        if (a[i] !== b[i]) return false;
    }

    return true;
}

export function isUrl(string: string): boolean {
    try {
        new URL(string);
    } catch {
        return false;
    }
    return true;
}
