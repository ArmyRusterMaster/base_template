#!/bin/bash
set -e

echo "🚀 ЗАПУСК ПОДГОТОВКИ ОКРУЖЕНИЯ РАЗРАБОТЧИКА (Rust + Leptos + Axum + Docker + Node + Сверхбыстрая сборка)..."

# --- 0. ОПРЕДЕЛЕНИЕ ДИСТРИБУТИВА И ПАКЕТНОГО МЕНЕДЖЕРА ---
if command -v apt-get &> /dev/null; then
    PM="apt"
    UPDATE_CMD="sudo apt-get update"
    INSTALL_CMD="sudo apt-get install -y"
    DOCKER_PKGS="docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin"
    NODE_PKGS="nodejs"
    # Для apt ставим lld и clang
    DEPS_PKGS="ca-certificates curl gnupg build-essential lld clang"
elif command -v pacman &> /dev/null; then
    PM="pacman"
    UPDATE_CMD="sudo pacman -Sy"
    INSTALL_CMD="sudo pacman -S --noconfirm"
    DOCKER_PKGS="docker docker-compose"
    NODE_PKGS="nodejs npm"
    # В Arch lld идет в комплекте или ставится отдельно
    DEPS_PKGS="base-devel curl lld clang"
elif command -v dnf &> /dev/null; then
    PM="dnf"
    UPDATE_CMD="sudo dnf check-update || true"
    INSTALL_CMD="sudo dnf install -y"
    DOCKER_PKGS="docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin"
    NODE_PKGS="nodejs"
    # Для Fedora/RHEL
    DEPS_PKGS="curl @development-tools lld clang"
elif command -v zypper &> /dev/null; then
    PM="zypper"
    UPDATE_CMD="sudo zypper refresh"
    INSTALL_CMD="sudo zypper install -y"
    DOCKER_PKGS="docker docker-compose"
    NODE_PKGS="nodejs npm"
    DEPS_PKGS="curl -t pattern devel_basis lld clang"
else
    echo "❌ Ошибка: Не удалось определить пакетный менеджер (поддерживаются apt, pacman, dnf, zypper)."
    exit 1
fi

echo "📦 Определен пакетный менеджер: $PM. Обновляем репозитории..."
$UPDATE_CMD

echo "🧰 Установка базовых инструментов сборки, компиляторов и линкера LLD..."
$INSTALL_CMD $DEPS_PKGS

# --- 1. ПРОВЕРКА И УСТАНОВКА DOCKER ---
if ! command -v docker &> /dev/null; then
    echo "🐳 Docker не найден. Устанавливаем официальный Docker Engine..."
    if [ "$PM" = "apt" ]; then
        sudo install -m 0755 -d /etc/apt/keyrings
        curl -fsSL https://docker.com | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
        sudo chmod a+r /etc/apt/keyrings/docker.gpg
        echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://docker.com $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
        sudo apt-get update
    elif [ "$PM" = "dnf" ]; then
        sudo dnf config-manager --add-repo https://docker.com || sudo dnf core-plugins-builddep-config-manager --add-repo https://docker.com
    fi
    
    $INSTALL_CMD $DOCKER_PKGS
    sudo systemctl enable --now docker
    sudo usermod -aG docker $USER
    echo "✅ Docker и плагин Docker Compose успешно установлены и запущены!"
else
    echo "✅ Docker уже установлен: $(docker --version)"
fi

# --- 2. УСТАНОВКА NODE.JS & NPM ---
if ! command -v node &> /dev/null; then
    echo "📦 Node.js не найден. Устанавливаем..."
    if [ "$PM" = "apt" ]; then
        curl -fsSL https://nodesource.com | sudo -E bash -
        $INSTALL_CMD nodejs
    elif [ "$PM" = "dnf" ]; then
        sudo dnf module enable -y nodejs:20 || true
        $INSTALL_CMD nodejs
    else
        $INSTALL_CMD $NODE_PKGS
    fi
    echo "✅ Node.js $(node -v) и npm $(npm -v) успешно установлены!"
else
    echo "✅ Node.js уже установлен: $(node -v)"
fi

# --- 3. УСТАНОВКА RUST (RUSTUP) ---
if ! command -v rustup &> /dev/null; then
    echo "🦀 Rust не найден. Устанавливаем через rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo "✅ Rust успешно установлен: $(rustc --version)"
else
    echo "✅ Rust уже установлен: $(rustc --version)"
fi

# --- 4. НАСТРОЙКА КОМПОНЕНТОВ RUST & ЭКОСИСТЕМЫ LEPTOS ---
echo "⚙️ Настройка компонентов Rust и утилит экосистемы..."

# Хелпер установки инструментов: сначала cargo-binstall (готовый бинарник —
# секунды вместо минут компиляции), при отсутствии/ошибке — обычный cargo install.
install_tool() {
    local bin="$1" crate="$2"
    if command -v "$bin" &> /dev/null; then
        echo "✅ $crate уже установлен."
        return
    fi
    echo "📦 Установка $crate..."
    if command -v cargo-binstall &> /dev/null; then
        cargo binstall -y "$crate" || cargo install --locked "$crate"
    else
        cargo install --locked "$crate"
    fi
}

# cargo-binstall — ускоритель установки Rust-инструментов (не критичен: при
# ошибке уходим на cargo install).
if ! command -v cargo-binstall &> /dev/null; then
    echo "📦 Установка cargo-binstall..."
    curl -L --proto '=https' --tlsv1.2 -sSf \
        https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
        | bash || echo "⚠️ cargo-binstall не установлен — используем cargo install."
fi

# 1. Установка WASM таргета
echo "🦀 Добавление WASM таргета..."
rustup target add wasm32-unknown-unknown

# 2. Форматирование и линтинг (те же компоненты, что гоняет CI)
rustup component add rustfmt clippy

# 3. Установка cargo-leptos для сборки и live-reload
install_tool cargo-leptos cargo-leptos

# 4. Установка форматировщика для макросов view!
install_tool leptosfmt leptosfmt

# 5. just — НЕ УСТАНАВЛИВАЕТСЯ локально (осознанное решение)
# justfile остаётся в репозитории как декларация задач; его роль на себя берёт CI
# (эквиваленты рецептов — это джобы fmt/clippy/test/build-release/e2e/docker).
# Локально используем прямые cargo-команды (docs/development.md → «Качество»).
# Если just всё же нужен — `cargo binstall -y just` вручную.

# 6. Установка утилит безопасности и аудита
install_tool cargo-audit cargo-audit

# 7. git-cliff — локальная генерация CHANGELOG (в CI используется action)
install_tool git-cliff git-cliff

# 8. TAILWIND CSS ОТДЕЛЬНО НЕ СТАВИТСЯ
# cargo-leptos сам скачивает standalone-бинарник Tailwind (версия задаётся
# LEPTOS_TAILWIND_VERSION, по умолчанию v4.x) и собирает style/main.css →
# target/site/pkg/base_template.css. Глобальная установка tailwind CLI не нужна
# и вредна: версия может разойтись с той, что ожидает cargo-leptos.

# 9. SCCACHE НЕ ИСПОЛЬЗУЕТСЯ
# sccache сознательно не ставится: в этом проекте обёртка компилятора отключена
# в .cargo/config.toml (rustc-wrapper = ""), т.к. sccache падает на Windows
# при компиляции web-sys (слишком длинная командная строка rustc).

# --- 5. УСТАНОВКА ЗАВИСИМОСТЕЙ ПРОЕКТА ---
if [ -f "end2end/package.json" ]; then
    echo "📦 Установка npm-зависимостей e2e-тестов (Playwright)..."
    (cd end2end && npm install)
fi

echo "✅ Окружение готово к инди-хакингу!"
echo "👉 Дальше: just watch (dev-сервер) или just --list (все рецепты)."
