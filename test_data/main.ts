import {$test} from "./inner/test";

export function queryUser(args: { text: string }) {
    $collection("user");
    $filter(user => user.name.includes(args.text));
    $test()
    $lookup("user", (item, result) => item.test2 = result, (root) => {
        $filter(user => {
            //let a = $sum((item) => 10);
            return root.name == user.name
        });
        $assign(() => {
            return {raw: true };
        });
    });
    $group(user => ({name: user.name, $id: user.$id}), (key) => {
        return {key, raw: $sum((item) => 10), data: $result() };
    })
}