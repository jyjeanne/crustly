#!/bin/bash
# Test if Ctrl keys are working in your terminal

echo "=== Testing Ctrl Key Detection ==="
echo ""
echo "This will test if your terminal sends Ctrl key events correctly."
echo ""
echo "Press these keys (then Ctrl+C to quit):"
echo "  1. Ctrl+A"
echo "  2. Ctrl+P"
echo "  3. Ctrl+R"
echo "  4. Just 'a' (without Ctrl)"
echo "  5. Ctrl+C to quit"
echo ""
echo "Watch what gets printed below:"
echo "------------------------------"

# Read key events
while IFS= read -rsn1 key; do
    # Print the key code
    printf "Key: '%s' (ASCII: %d)\n" "$key" "'$key"

    # Check for specific control characters
    case "$key" in
        $'\x01') echo "  → Detected: Ctrl+A" ;;
        $'\x10') echo "  → Detected: Ctrl+P" ;;
        $'\x12') echo "  → Detected: Ctrl+R" ;;
        $'\x03') echo "  → Detected: Ctrl+C (quitting)"; break ;;
        a) echo "  → Detected: Plain 'a' (no Ctrl)" ;;
        p) echo "  → Detected: Plain 'p' (no Ctrl)" ;;
        r) echo "  → Detected: Plain 'r' (no Ctrl)" ;;
    esac
done

echo ""
echo "=== Test Complete ==="
echo ""
echo "If you saw 'Plain' messages when pressing Ctrl+Key,"
echo "your terminal is NOT sending Ctrl key events correctly."
echo ""
echo "Possible fixes:"
echo "1. Check your terminal emulator settings"
echo "2. Try a different terminal (kitty, alacritty, gnome-terminal)"
echo "3. Check if your window manager is intercepting Ctrl keys"
echo "4. Try running in a basic terminal (tty)"
