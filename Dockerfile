FROM rust:1.69 as build

# create a new empty shell project
RUN USER=root cargo new --bin rust_workspace
WORKDIR /rust_workspace

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./migrations ./migrations

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/NoteTheLifeBackend*
WORKDIR /rust_workspace/src
RUN cargo build --release

# our final base
FROM rust:1.69

COPY ./migrations ./migrations

# copy the build artifact from the build stage
COPY --from=build /rust_workspace/target/release/NoteTheLifeBackend .

# set the startup command to run your binary
CMD ["./NoteTheLifeBackend"]
    