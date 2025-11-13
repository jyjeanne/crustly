#!/bin/bash
# Plan Mode Debug Script

echo "=== Crustly Plan Mode Diagnostics ==="
echo ""

echo "1. Current directory:"
pwd
echo ""

echo "2. Looking for plan JSON files:"
find /home/user -name ".crustly_plan_*.json" -type f 2>/dev/null | while read file; do
    echo "  Found: $file"
    echo "  Status: $(grep -o '"status":"[^"]*"' "$file" | cut -d'"' -f4)"
    echo "  Tasks: $(grep -o '"tasks":\[' "$file" -A 1 | wc -l)"
    echo ""
done

if [ ! -f /home/user -name ".crustly_plan_*.json" ]; then
    echo "  No plan JSON files found"
fi
echo ""

echo "3. Checking Crustly database:"
DB_PATH="$HOME/.local/share/crustly/crustly.db"
if [ -f "$DB_PATH" ]; then
    echo "  Database exists at: $DB_PATH"
    sqlite3 "$DB_PATH" "SELECT id, session_id, title, status, created_at FROM plans ORDER BY created_at DESC LIMIT 5;" 2>/dev/null || echo "  (Could not query database)"
else
    echo "  Database not found at: $DB_PATH"
fi
echo ""

echo "4. Recent Crustly log entries (if available):"
if [ -f "$HOME/.local/share/crustly/logs/crustly.log" ]; then
    tail -20 "$HOME/.local/share/crustly/logs/crustly.log" | grep -i "plan\|mode" | tail -5
else
    echo "  No log file found"
fi
echo ""

echo "=== End Diagnostics ==="
