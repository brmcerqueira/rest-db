declare function $collection(name: string): void;

declare function $filter<T = any>(match: (item: T) => boolean): void;

declare function $project<T = any, TResult = any>(match: (item: T) => TResult): void;

declare function $lookup<T = any, TLookup = any>(collection: string, destiny: ((item: T, result: TLookup[]) => void) | string, match: (item: T, itemLookup: TLookup) => boolean): void;