#!/bin/bash

echo "🤖 Building Macro Bot GUI..."
echo ""
echo "⏳ First build takes 5-10 minutes (compiling GUI framework)"
echo "   Subsequent builds are fast!"
echo ""

cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    echo "🚀 To run:"
    echo "   ./target/release/macro-bot-gui"
    echo ""
    echo "A window will open with all the controls!"
    echo "No more terminal blocking issues! 🎉"
    echo ""
else
    echo "❌ Build failed"
    exit 1
fi
