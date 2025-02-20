#!/bin/bash

zip -r queries.zip queries

curl -X PUT --location "http://localhost:8080/script/main" \
    -H "Content-Type: multipart/form-data" \
    -F "script=@queries.zip;filename=queries.zip;type=application/zip"
