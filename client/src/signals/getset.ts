import { Accessor, Setter, Signal } from "solid-js";

export interface GetSet<T> {
    get: Accessor<T>;
    set: Setter<T>;
}

export function getSet<T>(signal: Signal<T>): GetSet<T> {
    const [get, set] = signal;
    return { get, set };
}
