import {$test} from "./test";

export function test(args: { text: string }) {
    $collection("user");
    $filter(user => user.name.includes(args.text));
    $test()
    $lookup("user", (item, result) => item.test2 = result, (l, r) => l.$id == r.$id);
}