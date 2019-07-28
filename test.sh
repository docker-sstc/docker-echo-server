#!/bin/sh

HOST=localhost:3000

case "$1" in
	1)
		curl -v $HOST -d '{"a":123}'
	;;
	2)
		curl -v $HOST/ -d '{"a":123}'
	;;
	3)
		curl -v $HOST/_ -d '{"a":123}'
	;;
	4)
		curl -v $HOST/_/ -d '{"a":123}' # 404
	;;
	5)
		curl -v $HOST/_/foo -d '{"a":123}' # 404
	;;
	6)
		curl -v $HOST/_/version -d '{"a":123}'
	;;
	7)
		curl -v $HOST/bar.json -X POST -H 'content-type: application/json' \
-d '{"a":123}'
	;;
esac
