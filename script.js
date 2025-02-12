function name(args) {
    $collection("user");
    $filter(user => user.name == args.text);
}