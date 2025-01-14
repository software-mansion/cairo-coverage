#!/usr/bin/env bash
set -euxo pipefail

create_traces() {
  if [[ "$#" -ne 2 ]]; then
    echo "Usage: create_traces <origin-link> <commit-hash>"
    return 1
  fi

  local origin_link="$1"
  local commit_hash="$2"
  local repo_name
  repo_name=$(basename "$origin_link" .git)

  mkdir -p project-traces
  pushd project-traces

  # Initialize and fetch the repository
  git init "$repo_name"
  pushd "$repo_name"

  git fetch --depth 1 "$origin_link" "$commit_hash"
  git checkout "$commit_hash"

  # Run tests and generate trace data
  snforge test --save-trace-data

  # Prepare tracing directory
  mkdir -p trace
  find . -type d -name "snfoundry_trace" -exec cp -R {}/. ./trace \;

  # Clean up unnecessary files
  find . -mindepth 1 ! \( -path "./trace/*" -o -path "./target/dev/*" -o -name "trace" \) -delete

  popd
  popd
}


create_traces "git@github.com:starkware-libs/starknet-staking.git" "197e94c0cd10a11a44d261b27f2150c6aab3a25d"
