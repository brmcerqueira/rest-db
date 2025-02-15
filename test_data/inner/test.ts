import "../../rest-db"

export function $test() {
    $lookup("user", "test", (l, r) => l.$id == r.$id);
}