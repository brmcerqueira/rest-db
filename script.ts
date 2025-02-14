

function test(args: { text: string }) {
    $collection("user");
    $filter(user => user.name.includes(args.text));
    $lookup("user", "test", (l, r) => l.$id == r.$id);
    $lookup("user", (item, result) => item.test2 = result, (l, r) => l.$id == r.$id);
}