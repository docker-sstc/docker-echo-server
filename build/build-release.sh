#!/bin/bash

# from https://github.com/emk/rust-musl-builder/blob/master/examples/build-release

set -euo pipefail

USER=$1
NAME=$2
LABEL=$3

case `uname -s` in
	Linux)
		echo "Building static binaries using ekidd/rust-musl-builder"
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
