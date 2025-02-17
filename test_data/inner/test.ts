import "../../rest-db"

export function $test() {
    $lookup("user", "test", (root) => {
        $filter(user => root.$id == user.$id);
        $project(user => {
            return {$id:user.$id, full_name: user.name}
        });
    });
}