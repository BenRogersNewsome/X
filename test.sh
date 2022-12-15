#! /bin/bash

rm -rf ./coverage cov.profdata cov.profraw 

# Build tests and save json output
OUTPUT=$( \
          RUSTFLAGS="-C instrument-coverage" \
            cargo +nightly test --tests --no-run --message-format=json \
        ); \

# Search for the generated test executable in build output
[[ $OUTPUT =~ \"executable\":\"([^\"]+)\" ]]
TEST_FILE=${BASH_REMATCH[1]}

# Run tests
echo Running $TEST_FILE
LLVM_PROFILE_FILE="cov.profraw" eval $TEST_FILE

# Generate coverage
echo Generating coverage report
rust-profdata merge -sparse cov.profraw -o cov.profdata

COV_ARG=$( \
      for file in \
        $( \
        echo $OUTPUT \
              | jq -r "select(.profile.test == true)  | .filenames[]" \
              | grep -v dSYM - \
        ); \
      do \
        printf "%s %s " -object $file; \
      done \
    )

rust-cov report $COV_ARG \
  --instr-profile=cov.profdata --summary-only --ignore-filename-regex="/.cargo/registry|target"

# Save demangled output to HTML
mkdir ./coverage
touch ./coverage/index.html

rust-cov show $COV_ARG \
    --format=html --use-color --ignore-filename-regex='/.cargo/registry|target' \
    --instr-profile=cov.profdata \
    --show-instantiations --show-line-counts-or-regions \
    --Xdemangler=rustfilt | less -R > ./coverage/index.html