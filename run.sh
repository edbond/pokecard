#!/usr/bin/env bash

set -exou pipefail

pgrep chromedriver && pkill chromedriver

for port in $(seq 4000 4001); do
  echo starting browser on port "$port"
  ./chromedriver --port="$port" &
  sleep 1
done

# docker run --name some-scylla --hostname some-scylla -d scylladb/scylla --smp 1
# docker exec -it some-scylla cqlsh
