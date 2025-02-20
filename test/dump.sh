#!/bin/bash

URL="http://localhost:8080/collection"
JSON_FILE="json_data.json"
DELAY=10

insert() {
    local collection=$1
    local data=$2

    echo "Insert in: $collection"

    curl -X PUT \
        --header "Content-Type: application/json" \
        --data "$data" \
        "$URL/$collection"
}

cat "$JSON_FILE" | jq -r 'keys[]' | while read collection; do
    data=$(jq -r --arg collection "$collection" '.[$collection]' "$JSON_FILE")

    echo "$data" | jq -c '.[]' | while read item; do
        insert "$collection" "$item"

        echo "Waiting $DELAY milliseconds..."
        sleep $(echo "$DELAY / 1000" | bc -l)
    done
done