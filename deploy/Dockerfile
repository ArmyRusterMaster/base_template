# syntax=docker/dockerfile:1
# =============================================================================
# base_template — мультистейдж-сборка (Rust + Leptos + Axum)
#
#   build  : Ubuntu 24.04 + rustup. Сервер собирается под x86_64-unknown-linux-musl
#            → полностью статический бинарник, который можно запускать на Alpine.
#   run    : Alpine 3.20, минимальный рантайм, непривилегированный пользователь.
#
# Конфигурация сборки «подтягивается» из самого проекта:
#   * [package.metadata.leptos] в Cargo.toml — site-root, style-file, assets-dir,
#     фичи bin (`ssr`) / lib (`hydrate`), профиль wasm-release и т.д.
#   * config/app.yaml + переменные APP_* — рантайм-конфигурация приложения.
#
# Слои:
#   base    — тулачейн (Ubuntu + rustup + wasm/musl таргеты + cargo-chef + cargo-leptos)
#   planner — `cargo chef prepare`: рецепт зависимостей
#   deps    — `cargo chef cook`: компиляция/кеширование зависимостей (основной слой)
#   build   — `cargo-leptos build --release`
#   run     — Alpine: бинарник + site + конфиг
# =============================================================================

# ---------------------------------------------------------------------------
# STAGE 0: base
# ---------------------------------------------------------------------------
FROM ubuntu:24.04 AS base
ARG RUST_TOOLCHAIN=stable

ENV DEBIAN_FRONTEND=noninteractive \
    RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

# Базовые инструменты сборки + musl-tools (линковка musl-gcc).
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates curl build-essential pkg-config \
        musl-tools \
    && rm -rf /var/lib/apt/lists/*

# Rust + оба таргета: wasm (клиент/hydrate) и musl (сервер для Alpine).
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
        --profile minimal --default-toolchain "${RUST_TOOLCHAIN}" \
    && rustup target add wasm32-unknown-unknown x86_64-unknown-linux-musl

# Линковка musl-таргета через musl-gcc.
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc

# Утилиты Docker-сборки: chef (кеш зависимостей) и cargo-leptos (SSR+WASM+CSS).
RUN cargo install cargo-chef --locked \
    && cargo install cargo-leptos --locked

WORKDIR /app

# ---------------------------------------------------------------------------
# STAGE 1: planner — рецепт зависимостей
# ---------------------------------------------------------------------------
FROM base AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---------------------------------------------------------------------------
# STAGE 2: deps — кеш зависимостей (основной слой кеширования)
# ---------------------------------------------------------------------------
FROM base AS deps
COPY --from=planner /app/recipe.json recipe.json
# Собираем зависимости для фич серверной сборки и под musl-таргет.
RUN cargo chef cook --release --recipe-path recipe.json \
        --features ssr \
        --target x86_64-unknown-linux-musl

# ---------------------------------------------------------------------------
# STAGE 3: build — полная релиз-сборка
# ---------------------------------------------------------------------------
FROM base AS build
COPY --from=deps /app/target /app/target
COPY --from=planner /app/recipe.json recipe.json
COPY . .
# Таргет серверного бинарника — musl (статик, чтобы запускался на Alpine).
ENV LEPTOS_BIN_TARGET_TRIPLE=x86_64-unknown-linux-musl
RUN cargo-leptos build --release

# ---------------------------------------------------------------------------
# STAGE 4: runtime — Alpine
# ---------------------------------------------------------------------------
FROM alpine:3.20 AS runtime

# Минимальный рантайм: сертификаты + непривилегированный пользователь.
RUN apk add --no-cache ca-certificates tzdata \
    && addgroup -S app && adduser -S app -G app

# Рантайм-конфигурация Leptos: сайт лежит не в рабочей директории.
ENV LEPTOS_SITE_ROOT=/app/site \
    LEPTOS_SITE_PKG_DIR=pkg \
    APP_SERVER_HOST=0.0.0.0 \
    APP_SERVER_PORT=3000

WORKDIR /app

# Статика (JS/WASM/CSS), конфиг и исполняемый файл сервера.
COPY --from=build --chown=app:app /app/target/site ./site
COPY --from=build --chown=app:app /app/config ./config
COPY --from=build --chown=app:app \
     /app/target/x86_64-unknown-linux-musl/release/base_template \
     ./base_template

EXPOSE 3000

USER app
CMD ["/app/base_template"]