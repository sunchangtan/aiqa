#!/bin/bash
# 快速安全检测脚本（日常使用）
# 顺序：格式 → allow检测 → 质量 → 构建 → 测试 → 安全 → 许可证（快速失败原则）

set -euo pipefail

if [ ! -f Cargo.toml ]; then
  echo "❌ 请在 workspace 根目录（包含 Cargo.toml 的目录）执行此脚本"
  exit 1
fi

echo "⚡ 快速质量与安全检测..."

# 1. 代码格式检查（最快，最基础）
echo ""
echo "📐 1/7 检查代码格式..."
cargo fmt --all -- --check

# 2. 检测是否使用 #[allow(...)] / #![allow(...)] 关闭警告/错误
echo ""
echo "🚫 2/7 检测禁止的警告抑制..."

ALLOW_FOUND=$(
  grep -RIn --include="*.rs" \
    -E '^\s*#!?\[allow\(' \
    . \
    --exclude-dir=target || true
)

if [ -n "$ALLOW_FOUND" ]; then
    echo ""
    echo "❌ 错误：发现使用 #[allow(...)] / #![allow(...)] 抑制警告/错误！"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "$ALLOW_FOUND"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "💡 提示：不允许使用 #[allow(...)] 关闭警告或错误"
    echo "   请修复根本问题，而不是隐藏警告"
    echo ""
    echo "常见修复方法："
    echo "  - unused_variables: 添加 _ 前缀或重新设计接口"
    echo "  - dead_code: 删除未使用的代码或添加测试"
    echo "  - deprecated: 更新到新的API"
    exit 1
fi

# 3. 代码质量检查（在构建前发现问题）
echo ""
echo "🔧 3/7 代码质量检查 (clippy)..."
CLIPPY_FEATURES=${CLIPPY_FEATURES:-"--all-features"}
cargo clippy --all-targets "$CLIPPY_FEATURES" -- -D warnings

# 4. 构建检查（确保编译通过）
echo ""
echo "🔨 4/7 构建检查..."
cargo build --all-targets

# 5. 运行测试（确保功能正确，不允许忽略测试）
echo ""
echo "🧪 5/7 运行测试（严格模式：不允许 #[ignore]）..."
cargo test --workspace

echo ""
echo "🚫 扫描 #[ignore] 测试..."
IGNORED_ATTRS=$(
  grep -RIn --include="*.rs" \
    -E '^\s*#\[ignore' \
    . \
    --exclude-dir=target || true
)

if [ -n "$IGNORED_ATTRS" ]; then
    echo ""
    echo "❌ 错误：发现被忽略的测试（#[ignore]）！"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "$IGNORED_ATTRS"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "💡 提示：所有测试必须能够运行，不允许使用 #[ignore] 标记"
    echo "   请修复或删除被忽略的测试，确保代码质量"
    exit 1
fi

# 6. 安全漏洞检查（检查依赖安全性）
echo ""
echo "🛡️  6/7 检查安全漏洞..."
if [ "${SKIP_SECURITY:-0}" -ne 1 ]; then
  if command -v cargo-deny &> /dev/null; then
      cargo deny check advisories
  elif command -v cargo-audit &> /dev/null; then
      cargo audit
  else
      echo "⚠️  cargo-deny 和 cargo-audit 都未安装，跳过漏洞检测"
  fi
else
  echo "⚠️  SKIP_SECURITY=1，已跳过漏洞检测"
fi

# 7. 许可证合规检查（检查依赖许可证）
echo ""
echo "📋 7/7 检查许可证合规..."
if [ "${SKIP_SECURITY:-0}" -ne 1 ]; then
  if command -v cargo-deny &> /dev/null; then
      cargo deny check licenses
  else
      echo "⚠️  cargo-deny 未安装，跳过许可证检查"
  fi
else
  echo "⚠️  SKIP_SECURITY=1，已跳过许可证检查"
fi

echo ""
echo "✅ 所有检查通过！"
