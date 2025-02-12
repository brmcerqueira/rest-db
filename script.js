function name(args) {
    $collection("user");
    $filter(user => user.name.includes(args.text));
}