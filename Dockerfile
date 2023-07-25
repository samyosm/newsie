FROM rust:1.71 as build

# Create a new empty shell project
RUN USER=root cargo new --bin newsie 
WORKDIR /newsie

# Copy our manifests
COPY Cargo.lock Cargo.toml ./

# Build only the dependencies to cache them
RUN cargo build --release &&\
  rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/newsie* &&\
  cargo build --release

# The final base image
FROM debian:bullseye-slim

RUN apt-get update &&\
  apt-get install -y --no-install-recommends ca-certificates libxml2 &&\
  apt-get clean &&\
  rm -rf /var/lib/apt/lists/* &&\
  update-ca-certificates

# Copy from the previous build
COPY --from=build /newsie/target/release/newsie /usr/src/newsie
# COPY --from=build /holodeck/target/release/holodeck/target/x86_64-unknown-linux-musl/release/holodeck .

EXPOSE 8000 

# Run the binary
CMD ["/usr/src/newsie"]
