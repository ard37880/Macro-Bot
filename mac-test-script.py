#!/usr/bin/env python3
"""
Mac-compatible visual test script for macro-bot
Shows a notification and prints to confirm it's working
"""

import subprocess
import time
from datetime import datetime


def main():
    timestamp = datetime.now().strftime("%H:%M:%S")

    # Show Mac notification
    try:
        applescript = f"""
        display notification "Script is running! Time: {timestamp}" with title "🤖 Macro Bot Working"
        """
        subprocess.run(["osascript", "-e", applescript], check=True)
    except Exception as e:
        print(f"Notification error: {e}")

    # Print to console
    print(f"[{timestamp}] ✅ Script started")

    # Simulate work
    time.sleep(2)

    end_time = datetime.now().strftime("%H:%M:%S")
    print(f"[{end_time}] ✅ Script completed")

    return 0


if __name__ == "__main__":
    exit(main())
