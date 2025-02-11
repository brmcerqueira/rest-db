function $test(args) {
    $collection("user");
    $filter("teste!" + args.id + this);
}

function name(args) {
    $test(args);
}