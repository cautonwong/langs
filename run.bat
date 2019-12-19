@echo off
echo "====run rust app==="
cargo run main

echo "====run python app==="
python src\main.py

echo "====run go app==="
go run src\main.go

echo "====run js app==="
node src\main.js
