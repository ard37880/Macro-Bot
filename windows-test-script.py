#!/usr/bin/env python3
"""
Windows-compatible visual test script for macro-bot
Shows a notification popup so you can see it's working
"""

import sys
import time
from datetime import datetime


def show_windows_notification(title, message):
    """Show a Windows 10/11 toast notification"""
    try:
        from win10toast import ToastNotifier

        toaster = ToastNotifier()
        toaster.show_toast(title, message, duration=3, threaded=True)
    except ImportError:
        # Fallback to tkinter popup if win10toast not available
        try:
            import tkinter as tk
            from tkinter import messagebox

            root = tk.Tk()
            root.withdraw()  # Hide the main window
            root.attributes("-topmost", True)
            messagebox.showinfo(title, message)
            root.destroy()
        except:
            # Last resort - just print
            print(f"{title}: {message}")


def main():
    timestamp = datetime.now().strftime("%H:%M:%S")

    # Show notification
    show_windows_notification(
        "🤖 Macro Bot Working", f"Script is running!\nTime: {timestamp}"
    )

    # Print to console
    print(f"[{timestamp}] ✅ Script started")

    # Simulate work
    time.sleep(2)

    end_time = datetime.now().strftime("%H:%M:%S")
    print(f"[{end_time}] ✅ Script completed")

    return 0


if __name__ == "__main__":
    exit(main())
