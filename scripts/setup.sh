#!/usr/bin/env bash

set -e  # 遇到错误立即退出

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# 检查并安装系统依赖
install_system_deps() {
    info "检查系统依赖..."
    
    # 检查并安装 cmake
    if ! command -v cmake &> /dev/null; then
        info "安装 cmake..."
        brew install cmake
    else
        info "更新 cmake..."
        brew upgrade cmake
    fi

    # 检查并安装其他可能需要的系统依赖
    local deps=(pkg-config openssl)
    for dep in "${deps[@]}"; do
        if ! brew list $dep &>/dev/null; then
            info "安装 $dep..."
            brew install $dep
        else
            info "更新 $dep..."
            brew upgrade $dep
        fi
    done
}

# 检查是否安装了 Homebrew
check_brew() {
    info "检查 Homebrew..."
    if ! command -v brew &> /dev/null; then
        info "安装 Homebrew..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    else
        info "更新 Homebrew..."
        brew update
    fi
}

# 检查并安装 Rust
check_rust() {
    info "检查 Rust..."
    if ! command -v rustc &> /dev/null; then
        info "安装 Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    else
        info "更新 Rust..."
        rustup update
    fi
}

# 安装或更新必要的工具
install_tools() {
    info "安装/更新必要的工具..."
    
    # 安装/更新 git
    if ! command -v git &> /dev/null; then
        info "安装 git..."
        brew install git
    else
        info "更新 git..."
        brew upgrade git
    fi

    # 安装/更新项目所需的 Cargo 工具
    info "安装/更新 Cargo 工具..."
    for tool in cargo-edit cargo-smart-release git-cliff; do
        if ! command -v $tool &> /dev/null; then
            info "安装 $tool..."
            cargo install $tool
        else
            info "更新 $tool..."
            cargo install $tool --force
        fi
    done
}

# 更新项目依赖
update_project() {
    info "更新项目..."
    
    # 检查是否在 git 仓库中
    if ! git rev-parse --is-inside-work-tree &>/dev/null; then
        error "当前目录不是 git 仓库"
    fi
    
    # 保存当前分支
    current_branch=$(git symbolic-ref --short HEAD || echo "HEAD")
    
    # 检查是否有未提交的更改
    if ! git diff-index --quiet HEAD --; then
        warn "检测到未提交的更改，将它们暂存..."
        git stash
        local changes_stashed=true
    fi
    
    # 更新所有远程分支
    git fetch --all
    
    # 如果在 main 分支上，更新到最新
    if [ "$current_branch" = "main" ]; then
        git pull origin main
    else
        warn "当前不在 main 分支上，跳过更新"
    fi
    
    # 恢复暂存的更改
    if [ "$changes_stashed" = true ]; then
        warn "恢复暂存的更改..."
        git stash pop
    fi
}

# 设置 Git Hooks
setup_hooks() {
    info "设置 Git Hooks..."
    mkdir -p .hooks
    
    # 如果 pre-commit hook 不存在或内容不同，则更新它
    if [ ! -f .hooks/pre-commit ] || ! cmp -s .hooks/pre-commit .github/hooks/pre-commit 2>/dev/null; then
        info "更新 pre-commit hook..."
        cp .github/hooks/pre-commit .hooks/pre-commit 2>/dev/null || warn "pre-commit 模板不存在，跳过"
    fi
    
    chmod +x .hooks/pre-commit 2>/dev/null || warn "pre-commit hook 不存在，跳过"
    git config core.hooksPath .hooks
}

# 构建项目
build_project() {
    info "清理旧的构建文件..."
    cargo clean

    info "更新依赖..."
    cargo update

    info "构建项目..."
    cargo build

    info "运行测试..."
    cargo test
}

# 主函数
main() {
    info "开始设置/更新开发环境..."

    # 检查系统
    if [[ "$OSTYPE" != "darwin"* ]]; then
        error "此脚本仅支持 macOS"
    fi

    # 运行安装/更新步骤
    check_brew
    install_system_deps
    check_rust
    install_tools
    update_project
    setup_hooks

    # 构建项目
    build_project

    info "🎉 开发环境设置/更新完成！"
    info "你现在可以开始开发了。"
    info "提示："
    echo "1. 使用 'cargo run' 运行项目"
    echo "2. 使用 'cargo test' 运行测试"
    echo "3. 提交代码时会自动运行检查"
    echo "4. 使用 conventional commits 格式提交代码"
    echo "5. 随时可以重新运行此脚本来更新环境"
}

# 运行主函数
main 