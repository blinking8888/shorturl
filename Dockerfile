# Define build-time arguments
ARG RUST_VERSION=1.77.0
ARG PORT=7777
ARG SHORTURL_LENGTH=5

# Stage 1: Build the Application
FROM rust:${RUST_VERSION}-slim-bullseye AS builder

# Set environment variables
ENV RUST_VERSION=$RUST_VERSION

# Create a work directory
WORKDIR /app

# Install build dependencies
RUN --mount=type=cache,target=/var/cache/apt \
    apt-get update && apt-get install -y build-essential cmake pkg-config libssl-dev git curl

ENV CARGO_HOME=/.cargo
RUN mkdir -p $CARGO_HOME
RUN mkdir -p /app/build

# Build the Rust application
RUN --mount=type=bind,source=./,target=/usr/src/app \
    --mount=type=cache,target=$CARGO_HOME \
    cd /usr/src/app && cargo build --target-dir /app/build --locked --release

# Stage 2: Create a Deployment Image
FROM debian:bullseye-slim AS deployment

RUN --mount=type=cache,target=/var/cache/apt \
    apt-get update && \
    apt-get install --no-install-recommends -y ca-certificates && \
    apt-get clean

VOLUME /data

# Create a work directory
WORKDIR /app

# Set the user as the owner of the application directory
RUN chown -R nobody /app

# Switch to the non-root user
USER nobody

# Set the environment variable with the application name
ARG PORT=$PORT
ENV PORT=$PORT
ARG SHORTURL_LENGTH=$SHORTURL_LENGTH
ENV SHORTURL_LENGTH=$SHORTURL_LENGTH

# Copy the built application
COPY --from=builder /app/build/release/shorturl /app/shorturl

EXPOSE ${PORT}

ENTRYPOINT [ "sh", "-c", "/app/shorturl" ]
CMD [ "-p $PORT -s $SHORTURL_LENGTH -d /app/data/shorturls.db" ]
