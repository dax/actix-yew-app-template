FROM lukemathwalker/cargo-chef:latest-rust-1.57.0 as chef
WORKDIR /app

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef as dep-builder
RUN cargo install cargo-make
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook -p {{project-name}} --release --recipe-path recipe.json
RUN cargo chef cook -p {{project-name}}-api --release --recipe-path recipe.json
RUN cargo chef cook -p {{project-name}}-web --release --recipe-path recipe.json --target wasm32-unknown-unknown

FROM dep-builder as release-builder
COPY . .
RUN cargo make build-release
RUN sed -i 's#http://localhost:8000/api#/api#' web/dist/snippets/{{project-name}}-web-*/js/api.js

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN mkdir /data
COPY --from=release-builder /app/target/release/{{project-name}}-api {{project-name}}
COPY --from=release-builder /app/api/config/default.toml config/default.toml
COPY --from=release-builder /app/web/dist/ .
ENV {{crate_name | upcase}}_APPLICATION.API_PATH /api
ENV {{crate_name | upcase}}_APPLICATION.STATIC_PATH /
CMD ["/app/{{project-name}}"]
