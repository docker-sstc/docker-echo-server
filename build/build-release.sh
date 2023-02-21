#!/bin/bash

set -euo pipefail

URL=https://github.com/rust-cross/rust-musl-cross
USER=$1
NAME=$2
LABEL=$3

case $(uname -s) in
Linux)
	echo "Building static binaries using $URL"
	image="$USER/$NAME"
	docker build -t "$image" .
	docker run -d --name c-"$NAME" "$image"
	docker stop c-"$NAME"
	docker cp c-"$NAME":/app/main "$NAME"
	docker rm c-"$NAME"
	# docker rmi "$image"
	zip "$NAME"-"$LABEL".zip "$NAME"
	ls -al
	;;
*)
	echo "Building standard release binaries"
	cargo build --release
	zip -j "$NAME"-"$LABEL".zip target/release/"$NAME"
	ls -al
	;;
esac
