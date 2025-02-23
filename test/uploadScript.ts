#!/usr/bin/env -S deno run --allow-read --allow-write --allow-run --allow-net
import { compress } from "jsr:@fakoua/zip-ts";

const filePath = "queries.zip";

await compress("queries", filePath, { overwrite: true });

const fileContent = await Deno.readFile(filePath);

const fileBlob = new Blob([fileContent], { type: "application/zip" });

const formData = new FormData();
formData.append("script", fileBlob, "queries.zip");

const response = await fetch("http://localhost:8080/script/main", {
    method: "PUT",
    body: formData,
});

if (response.ok) {
    console.log("Successfully uploaded script");
} else {
    console.error(`Failed to upload script: ${response.statusText}`);
}
