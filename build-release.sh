#!/bin/bash
#
# Usage: ./build-release <PROJECT> ${TRAVIS_TAG}-${TRAVIS_OS_NAME}
#
# The latest version of this script is available at
# https://github.com/emk/rust-musl-builder/blob/master/examples/build-release
#
# Called by `.travis.yml` to build release binaries.  We use
# ekidd/rust-musl-builder to make the Linux binaries so that we can run
# them unchanged on any distro, including tiny distros like Alpine (which
# is heavily used for Docker containers).  Other platforms get regular
# binaries, which will generally be dynamically linked against libc.
#
# If you have a platform which supports static linking of libc, and this
# would be generally useful, please feel free to submit patches.

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
