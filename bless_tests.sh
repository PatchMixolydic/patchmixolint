#!/bin/sh

echo collecting stderr output...
cargo test >> /dev/null
sleep 2

# TODO: automatic test discovery
echo moving updated stderr output...

mv_stderr() {
    if test -f /tmp/$1.stage-id.stderr; then
        rm ui/$1.stderr
        mv /tmp/$1.stage-id.stderr ui/$1.stderr
    fi
}

mv_stderr early_pass
mv_stderr late_pass
mv_stderr missing_lints_forbid_unsafe

echo cargo test...
cargo test
