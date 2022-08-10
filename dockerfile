FROM rust:1.62.1 as build

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

# create a new empty shell project
RUN USER=root cargo new --bin f1-ical
WORKDIR /f1-ical

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
#RUN rm ./target/release/deps/f1-ical*
RUN cargo build --release

# our final base
FROM rust:1.62.1-slim-buster

# copy the build artifact from the build stage
COPY --from=build /f1-ical/target/release/f1-ical .

# set the startup command to run your binary
CMD ["./f1-ical"]
