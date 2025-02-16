import "../../rest-db"

export function $test() {
    $lookup("user", "test", (l, r) => l.$id == r.$id);
    $project(user => {
        return {$id:user.$id, full_name: user.name, test: user.test}
    });
}