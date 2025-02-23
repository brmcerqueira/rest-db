#!/usr/bin/env -S deno run --allow-read --allow-write --allow-run --allow-net
const filePath = "queries.zip";

Deno.run({
    cmd: ["zip", "-r", filePath, "queries"],
});

const file = await Deno.open(filePath);

const formData = new FormData();

formData.append("script", file.readable, "queries.zip");

await fetch("http://localhost:8080/script/main", {
    method: "PUT",
    body: formData,
    headers: {
        "Content-Type": "multipart/form-data",
    },
});