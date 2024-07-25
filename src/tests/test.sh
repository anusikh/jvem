#!/bin/bash

assert() {
  local output="$1"
  local expected_output="$2"
  if [ "$output" == "$expected_output" ]; then
    echo "[passed]"
  else
    echo "[failed]"
    exit 1
  fi
}

starts_with() {
  local output="$1"
  local start="$2"
  if [[ $output =~ ^"$start" ]]; then
    echo "[passed]"
  else
    echo "[failed]"
  fi
}

# node version management test
echo "node version management tests....."
target/release/jvem node install 21.7.3
target/release/jvem node install 22.5.0
output=$(target/release/jvem node ls)
expected_output=$(
cat <<EOF
22.5.0
21.7.3
EOF
)
assert "$output" "$expected_output"

target/release/jvem node usev 21.7.3
output=$(~/.jvem/node/bin/node -v)
expected_output="v21.7.3"
assert "$output" "$expected_output"

target/release/jvem node usev 22.5.0
output=$(~/.jvem/node/bin/node -v)
expected_output="v22.5.0"
assert "$output" "$expected_output"

target/release/jvem node uninstall 21.7.3
output=$(target/release/jvem node ls)
expected_output=$(
cat <<EOF
22.5.0
EOF
)
assert "$output" "$expected_output"

# java version management test
echo "java version management tests....."
target/release/jvem java install openjdk22
target/release/jvem java install openjdk17
target/release/jvem java usev openjdk17
output=$(~/.jvem/java/bin/java --version)
expected_output=$(
cat <<EOF
openjdk 17.0.2 2022-01-18
OpenJDK Runtime Environment (build 17.0.2+8-86)
OpenJDK 64-Bit Server VM (build 17.0.2+8-86, mixed mode, sharing)
EOF
)
assert "$output" "$expected_output"

target/release/jvem java usev openjdk22
output=$(~/.jvem/java/bin/java --version)
expected_output=$(
cat <<EOF
openjdk 22.0.1 2024-04-16
OpenJDK Runtime Environment (build 22.0.1+8-16)
OpenJDK 64-Bit Server VM (build 22.0.1+8-16, mixed mode, sharing)
EOF
)
assert "$output" "$expected_output"


# maven version management
echo "node version management tests....."
target/release/jvem maven install
output=$(~/.jvem/maven/bin/mvn --version)
start="Apache Maven 3.9.8"
starts_with "$output" "$start"

output=$(target/release/jvem maven uninstall)
expected_output="maven uninstall successful"
assert "$output" "$expected_output"
