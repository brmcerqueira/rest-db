import {$test} from "./inner/test";

export function queryUser(args: { text: string }) {
    $collection("user");
    $filter(user => user.name.includes(args.text));
    $test()
    $lookup("user", (item, result) => item.test2 = result, function (root) {
        $filter(user => root.$id == user.$id);
    });
}