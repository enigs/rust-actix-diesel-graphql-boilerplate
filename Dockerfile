# Stage 1: Generate a recipe file
FROM rust:latest as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Build dependencies
FROM planner as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Stage 3: Create builder
FROM cacher as build

# Update and upgrade builder container
#RUN apt update && apt upgrade -y && apt-get install libclang-dev -y
RUN apt update && apt upgrade -y

# Copy directory and build
WORKDIR /app

# Copy the app into docker image
COPY . /app

# Copy dependencies so that it can skip rebuilding dependencies that was already cached before
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# Build release
RUN cargo build --release

# Our final base
FROM rust:slim

RUN apt update && apt-get install --yes libpq5

# copy the build artifact from the build stage
COPY --from=build /app/target/release/server /server
COPY --from=build /app/assets /assets

# set the startup command to run your binary
ENTRYPOINT ["/server"]
EXPOSE 9020