#!/bin/bash
# Test Plan Mode functionality

set -e

echo "=== Plan Mode Test Script ==="
echo ""

# Create a test directory
TEST_DIR="/tmp/crustly_plan_test_$$"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

echo "1. Test directory: $TEST_DIR"
echo "2. Building Crustly (if needed)..."

cd /home/user/crustly
if [ ! -f target/release/crustly ]; then
    echo "   Building release binary..."
    cargo build --release
fi

echo "3. Copying binary to test directory..."
cp target/release/crustly "$TEST_DIR/"

cd "$TEST_DIR"

echo "4. Starting Crustly with debug logging..."
echo ""
echo "============================================"
echo "INSTRUCTIONS:"
echo "============================================"
echo "1. Wait for Crustly to start"
echo "2. Type: 'create a plan to add a hello world function'"
echo "3. Press Ctrl+Enter to send"
echo "4. Watch for the message: 'Plan finalized!'"
echo "5. Check if the mode switches from [CHAT] to [PLAN]"
echo "6. If in [PLAN] mode:"
echo "   - Press Ctrl+A to approve"
echo "   - OR Ctrl+R to reject"
echo "   - OR Esc to cancel"
echo "7. Press Ctrl+C to quit"
echo ""
echo "After running, check this directory for:"
echo "  - .crustly_plan_*.json files"
echo "  - PLAN.md (if you approved)"
echo ""
echo "Logs will be in: $TEST_DIR/test.log"
echo "============================================"
echo ""

read -p "Press Enter to start Crustly..."

RUST_LOG=crustly=debug,info ./crustly chat 2>&1 | tee test.log
