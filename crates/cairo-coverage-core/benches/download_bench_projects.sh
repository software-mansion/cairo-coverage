#!/usr/bin/env bash
set -euxo pipefail

create_traces() {
  if [[ "$#" -ne 4 ]]; then
    echo "Usage: create_traces <origin-link> <commit-hash> <starknet-foundry-version> <scarb-version>"
    return 1
  fi

  local origin_link="$1"
  local commit_hash="$2"
  local foundry_version="$3"
  local scarb_version="$4"
  local repo_name
  repo_name=$(basename "$origin_link" .git)

  mkdir -p project-traces
  pushd project-traces

  # Initialize and fetch the repository
  git init "$repo_name"
  pushd "$repo_name"

  git fetch --depth 1 "$origin_link" "$commit_hash"
  git checkout "$commit_hash"

  # Create .tool-versions file for the project
  echo "starknet-foundry ${foundry_version}" > .tool-versions
  echo "scarb ${scarb_version}" >> .tool-versions

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

create_traces "git@github.com:starkware-libs/starknet-staking.git" "b37679f653bd7ef58cfdf2eb434dd10569f89ea1" 0.49.0 2.12.2
