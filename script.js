function $test(args, data) {
    $collection("user");
    $filter(data.name + args.id + this);
}

function name(args) {
    $test(args, JSON.parse("{\"name\":\"test\"}"));
}