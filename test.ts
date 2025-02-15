import "./rest-db.d.ts"
export function $test() {
    $lookup("user", "test", (l, r) => l.$id == r.$id);
}