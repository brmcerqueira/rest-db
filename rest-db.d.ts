declare function $collection(name: string): void;

declare function $filter<T = any>(match: (item: T) => boolean): void;

declare function $project<T = any, TResult = any>(transform: (item: T) => TResult): void;

declare function $assign<T = any, TResult = any>(add: (item: T) => TResult): void;

declare function $lookup<T = any, TLookup = any>(collection: string, destiny: ((item: T, result: TLookup[]) => void) | string, scope?: (item: T) => void): void;

declare function $group<T = any>(transform: () => T): void;

declare function $group<T = any, TKey = any, TResult = any>(key: (T) => TKey, transform: (TKey) => TResult): void;

declare function $result<T = any>(): T[];

declare function $sum<T = any, TResult = any>(expression: (T) => TResult): TResult;