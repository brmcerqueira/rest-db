declare function $collection(name: string): void;

declare function $filter<T = any>(match: (item: T) => boolean): void;

declare function $project<T = any, TResult = any>(transform: (item: T) => TResult): void;

declare function $lookup<T = any>(collection: string, destiny: ((item: T) => void) | string, scope?: (item: T) => void): void;

declare function $group<T = any>(transform: () => T): void;

declare function $group<T = any, TKey = any, TResult = any>(key: (T) => TKey, transform: (TKey) => TResult): void;

declare function $all<T = any>(): T[];

declare function $first<T = any>(): T;

declare function $last<T = any>(): T;

declare function $sum<T = any, TResult = any>(callback: (T) => TResult): TResult;