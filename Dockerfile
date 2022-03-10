FROM golang:1.17-buster as build
RUN apt-get update -y && apt-get upgrade -y && apt-get install -y libleveldb-dev
RUN apt-get install -y unzip

RUN echo "Running Test"
COPY --chown=0:0 scripts/simple_test.sh simple_test.sh

# Initialize provenance to run with the default node configuration
CMD ["./simple_test.sh"]