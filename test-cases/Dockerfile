ARG BASE_IMAGE
ARG RUN_IMAGE

FROM $BASE_IMAGE AS builder

ARG TEST_CASE
ARG RUST_FLAGS

WORKDIR /test
COPY TEST_CASE .
RUN RUSTFLAGS=$RUST_FLAGS cargo build --release

FROM $RUN_IMAGE
COPY --from=builder /test/target/release/test /test
CMD ["/test"]
