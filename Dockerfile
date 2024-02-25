FROM rust:1.69 as build

ARG POSTGRES_USER
ARG POSTGRES_PORT
ARG POSTGRES_DATABASE
ARG POSTGRES_HOST
ARG ROCKET_ADDRESS

# create a new empty shell project
RUN USER=root cargo new --bin rust_workspace
WORKDIR /rust_workspace

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./migrations ./migrations

ARG POSTGRES_USER
ENV POSTGRES_USER=${POSTGRES_USER}
ARG POSTGRES_PORT
ENV POSTGRES_PORT=${POSTGRES_PORT}
ARG POSTGRES_DATABASE
ENV POSTGRES_DATABASE=${POSTGRES_DATABASE}
ARG POSTGRES_HOST
ENV POSTGRES_HOST=${POSTGRES_HOST}
ARG ROCKET_ADDRESS
ENV ROCKET_ADDRESS=${ROCKET_ADDRESS}

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/note_the_life_backend*
WORKDIR /rust_workspace/src
RUN cargo build --release

# our final base
FROM rust:1.69

COPY ./migrations ./migrations

ARG POSTGRES_USER
ENV POSTGRES_USER=${POSTGRES_USER}
ARG POSTGRES_PORT
ENV POSTGRES_PORT=${POSTGRES_PORT}
ARG POSTGRES_DATABASE
ENV POSTGRES_DATABASE=${POSTGRES_DATABASE}
ARG POSTGRES_HOST
ENV POSTGRES_HOST=${POSTGRES_HOST}
ARG ROCKET_ADDRESS
ENV ROCKET_ADDRESS=${ROCKET_ADDRESS}

# copy the build artifact from the build stage
COPY --from=build /rust_workspace/target/release/note_the_life_backend .
EXPOSE 8000
# set the startup command to run your binary
CMD ["./note_the_life_backend"]
    