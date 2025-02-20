import {$test} from "./inner/test";

export function queryUser(args: { text: string }) {
    $collection("Category");
    $filter(category => category.description.includes(args.text));
}