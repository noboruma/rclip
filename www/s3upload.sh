aws s3 sync ./dist s3://www.remote-clipboard.net --include "*" --exclude "*.wasm" --delete --acl public-read
aws s3 sync ./dist s3://www.remote-clipboard.net --exclude "*" --include "*.wasm" --content-type "application/wasm" --delete --acl public-read
