# Docker file for testing provwasm
FROM golang:1.17-buster as build
RUN apt-get update -y && apt-get upgrade -y && apt-get install -y libleveldb-dev
RUN apt-get install -y unzip
ARG test_script
ARG contract_location
ARG contract_destination

COPY --chown=0:0 ./scripts/simple_test.sh simple_test.sh
COPY --chown=0:0 "$test_script" "$test_script"
COPY "$contract_location" "$contract_destination"

# install jq for parsing output of queries
RUN curl -o /usr/local/bin/jq http://stedolan.github.io/jq/download/linux64/jq && \
  chmod +x /usr/local/bin/jq

# Initialize provenance to run with the default node configuration
ENTRYPOINT ["./simple_test.sh"]