FROM rust as base

LABEL maintainer="adorsys Cameroon"

WORKDIR /app

FROM base as builder

COPY ./ ./

RUN cargo build --release

FROM base

WORKDIR /app

COPY --from=builder /app/target/release/applicant_selection_app /app/applicant_selection_app

EXPOSE 8080

CMD ["/app/applicant_selection_app"]
