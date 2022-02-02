#!/bin/sh

while :; do
	inotifywait -e close_write,moved_to,create -rq . 
	clear
	cargo test
done
