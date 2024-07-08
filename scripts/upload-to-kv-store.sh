#!/bin/bash
STORE_ID=$1

# Directory containing precompiled files.
PRECOMPILED_DIR="$__dir/../data/precompiled"

# Concurrently insert data into a KV Store.
# The filename will be used as the key, and the file contents will be used as the value.
fastly kv-store-entry create --store-id="$STORE_ID" --dir $PRECOMPILED_DIR

# Deleting old keys from KV Store:
# for i in {0..499}
# do
#    fastly kv-store-entry delete --store-id="$STORE_ID" --key="precompiled/cluster-map-$i.bincode"
# done

# Deleting all keys from KV Store:
# fastly kv-store-entry delete --store-id="$STORE_ID" --all