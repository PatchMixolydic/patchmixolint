#!/bin/sh

echo collecting stderr output...
cargo test >> /dev/null
sleep 2

# TODO: automatic test discovery
echo copying updated stderr output...
cp /tmp/early_pass.stage-id.stderr ui/early_pass.stderr
cp /tmp/late_pass.stage-id.stderr ui/late_pass.stderr

echo cargo test...
cargo test
