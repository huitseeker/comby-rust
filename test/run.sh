#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

pushd $DIR > /dev/null

prefixes=( templates nopanic experimental-templates )

for p in "${prefixes[@]}"
do
    comby -config "../$p.toml" -f "$p-test.rs" -stdout > "$p-test.tmp.rs"

    diff "$p-test.tmp.rs" "$p-test.expect.rs"
    ret=$?
    if [ $ret -ne 0 ]; then
        echo "diff output for $p is wrong"
        echo "update with: comby -config ../$p.toml -f $p-test.rs -stdout > $p-test.expect.rs"
        rm "$p-test.tmp.rs"
        exit 1
    fi

    rustfmt "$p-test.tmp.rs" > /dev/null

    ret=$?
    if [ $ret -ne 0 ]; then
        echo "rustfmt should succeed"
        rm $p-test.tmp.rs
        exit 1
    fi

    rm "$p-test.tmp.rs"
done

popd > /dev/null
