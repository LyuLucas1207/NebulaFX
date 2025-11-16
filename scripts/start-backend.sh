#!/bin/sh
# 启动 RustFS 后端（不启用嵌入的 Console）

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# 加载 Rust 环境
if [ -f "../../use-rust1.91.sh" ]; then
    source ../../use-rust1.91.sh
fi

# 设置环境变量
export RUSTFS_VOLUMES="${RUSTFS_VOLUMES:-./deploy/data/dev{1...8}}"
export RUSTFS_ADDRESS="${RUSTFS_ADDRESS:-:9000}"
# 已移除：不再需要 RUSTFS_CONSOLE_ENABLE 和 RUSTFS_CONSOLE_ADDRESS
# Console API 端点始终通过主服务器提供

# 日志配置
if [ -z "$RUST_LOG" ]; then
    export RUST_LOG="rustfs=info,ecstore=info,s3s=info,iam=info"
fi
# 启用 stdout 日志输出（默认输出到控制台）
export RUSTFS_OBS_LOGGER_LEVEL="${RUSTFS_OBS_LOGGER_LEVEL:-info}"
export RUSTFS_OBS_LOG_STDOUT_ENABLED="${RUSTFS_OBS_LOG_STDOUT_ENABLED:-true}"
# 日志格式：false=文本格式（易读），true=JSON格式（用于日志收集系统）
export RUSTFS_LOG_JSON="${RUSTFS_LOG_JSON:-false}"

echo "🚀 启动 RustFS 后端..."
echo "   地址: ${RUSTFS_ADDRESS}"
echo "   Console API: /rustfs/console/* (始终可用)"
echo "   前端: 独立运行（rustfsconsole 项目）"
echo ""

# 运行后端
cargo run --bin rustfs

