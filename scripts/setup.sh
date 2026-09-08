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

# 1. Установка WASM таргета
echo "🦀 Добавление WASM таргета..."
rustup target add wasm32-unknown-unknown

# 2. Установка cargo-leptos для сборки и live-reload
if ! command -v cargo-leptos &> /dev/null; then
    echo "📦 Установка cargo-leptos..."
    cargo install --locked cargo-leptos
else
    echo "✅ cargo-leptos уже установлен."
fi

# 3. Установка форматировщика для макросов view!
if ! command -v leptosfmt &> /dev/null; then
    echo "📦 Установка leptosfmt..."
    cargo install --locked leptosfmt
else
    echo "✅ leptosfmt already installed."
fi

# 4. Установка утилит безопасности и аудита
if ! command -v cargo-audit &> /dev/null; then
    echo "📦 Установка cargo-audit..."
    cargo install --locked cargo-audit
else
    echo "✅ cargo-audit уже установлен."
fi

# 5. УСТАНОВКА И ГЛОБАЛЬНАЯ НАСТРОЙКА SCCACHE
if ! command -v sccache &> /dev/null; then
    echo "📦 Установка sccache (кэш компиляции)..."
    cargo install --locked sccache
else
    echo "✅ sccache уже установлен."
fi

# Интеграция sccache в профиль пользователя, чтобы он работал глобально во всех проектах
SHELL_RC=""
if [ -f "$HOME/.bashrc" ]; then SHELL_RC="$HOME/.bashrc"; fi
if [ -f "$HOME/.zshrc" ]; then SHELL_RC="$HOME/.zshrc"; fi

if [ -n "$SHELL_RC" ]; then
    if ! grep -q "RUSTC_WRAPPER" "$SHELL_RC"; then
        echo -e "\n# Глобальный кэш компиляции Rust\nexport RUSTC_WRAPPER=sccache" >> "$SHELL_RC"
        echo "📝 Переменная RUSTC_WRAPPER добавленна в $SHELL_RC"
    fi
fi

# --- 5. УСТАНОВКА ЗАВИСИМОСТЕЙ ПРОЕКТА ---
if [ -f "package.json" ]; then
    echo "📦 Файл package.json найден. Синхронизируем node-зависимости (Tailwind и др.)..."
    npm install
fi

echo "✅ Окружение готово к инди-хакингу с супер-быстрой сборкой!"
echo "💡 Чтобы применить настройки sccache прямо сейчас, выполните: source $SHELL_RC"
