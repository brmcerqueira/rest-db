#!/usr/bin/env -S deno run --allow-read --allow-net
const DELAY = 10;

const parsedData = JSON.parse(await Deno.readTextFile("./json_data.json"));

for (const collection in parsedData) {
    const data = parsedData[collection];

    for (const item of data) {
        console.log(`Insert in: ${collection}`);

        const response = await fetch(`http://localhost:8080/collection/${collection}`, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(item),
        });

        if (response.ok) {
            console.log(`Successfully inserted data into ${collection}`);
        } else {
            console.error(`Failed to insert data into ${collection}: ${response.statusText}`);
        }

        console.log(`Waiting ${DELAY} milliseconds...`);

        await new Promise((resolve) => setTimeout(resolve, DELAY));
    }
}
