FROM golang:1.17-buster as build
ARG VERSION

WORKDIR /go/src/github.com/provenance-io/provenance
ENV GOPRIVATE=github.com/provenance-io
RUN apt-get update && apt-get upgrade -y && apt-get install -y libleveldb-dev

RUN git clone https://github.com/provenance-io/provenance.git

# Build the binaries
RUN go build -mod vendor \
    -tags cleveldb \
    -ldflags '-w -s -X github.com/cosmos/cosmos-sdk/version.Name=Provenance-Blockchain' \
    -o /go/bin/ ./cmd/...
RUN echo "Completed go build"

RUN echo "Initializing provenance"

# Initialize provenance to run with the default node configuration
WORKDIR /testrosetta
RUN provenanced testnet -t --v 1 -o . --starting-ip-address=0.0.0.0 --keyring-backend=test --chain-id=chain-local