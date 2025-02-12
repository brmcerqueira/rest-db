function name(args) {
    $collection("user");
    $filter(user => user.name.includes(args.text));
    $lookup("user", "test", (l, r) => l.$id == r.$id);
}