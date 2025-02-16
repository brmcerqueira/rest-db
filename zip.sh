#!/bin/bash

zip -r test_data.zip test_data

curl -X PUT --location "http://localhost:8080/script/main" \
    -H "Content-Type: multipart/form-data" \
    -F "script=@test_data.zip;filename=test_data.zip;type=application/zip"
