# Macro Bot - Automated Script Runner by OtterBot

A cross-platform GUI application that continuously runs scripts with configurable delays. Perfect for automation tasks, testing, and repetitive script execution.

![Platform Support](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-blue)
![Language](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-green)

---

## 🌟 Features

✅ **Cross-Platform** - Works on macOS and Windows  
✅ **Beautiful GUI** - Clean, modern window interface  
✅ **Continuous Execution** - Automatically restarts scripts when they complete  
✅ **Configurable Delay** - Set custom wait time between script runs (1-3600 seconds)  
✅ **Multiple Script Types** - Supports `.py`, `.sh`, `.bat`, `.cmd`, `.exe`, and more  
✅ **Non-Blocking** - Won't freeze your system or interfere with other applications  
✅ **Run Counter** - Track how many times your script has executed  
✅ **Simple Controls** - Start/Stop with easy button clicks  
✅ **Status Display** - See real-time script status and messages  

---

## 📋 Requirements

### macOS
- **macOS 10.15+** (Catalina or later)
- **Rust** (1.70 or later)
- **Python 3** (if running Python scripts)

### Windows
- **Windows 10/11**
- **Rust** (1.70 or later)
- **Visual Studio Build Tools** (for compiling)
- **Python 3** (if running Python scripts)

---

## 🚀 Installation

### Step 1: Install Rust

#### macOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### Windows
1. Download from: https://rustup.rs/
2. Run `rustup-init.exe`
3. Press Enter to proceed with default installation
4. Restart your terminal

Verify installation:
```bash
cargo --version
```

### Step 2: Install Visual Studio Build Tools (Windows Only)

1. Download **Build Tools for Visual Studio 2022** from:
   https://visualstudio.microsoft.com/downloads/
2. Run installer and select **"Desktop development with C++"**
3. Install (~6GB, takes 10-15 minutes)
4. Restart your computer

### Step 3: Build Macro Bot

#### macOS
```bash
cd macro-bot
chmod +x build.sh
./build.sh
```

#### Windows
```cmd
cd macro-bot
build.bat
```

**Note:** First build takes **5-10 minutes** (compiling GUI framework). Subsequent builds are fast!

### Step 4: Run It!

#### macOS
```bash
./target/release/macro-bot-gui
```

#### Windows
```cmd
.\target\release\macro-bot-gui.exe
```

---

## 📖 How to Use

### Quick Start Guide

1. **Launch the Application**
   - macOS: `./target/release/macro-bot-gui`
   - Windows: `.\target\release\macro-bot-gui.exe`

2. **Select Your Script**
   - Click the **"Select Script"** button
   - Browse to your script file
   - Supported types: `.py`, `.sh` (Mac), `.bat`/`.cmd` (Windows), `.exe` (Windows)

3. **Set Delay (Optional)**
   - Type the number of seconds to wait between script runs
   - Default: 10 seconds
   - Range: 1-3600 seconds (1 hour)

4. **Start Running**
   - Click **"▶ Start Running"**
   - The script will execute continuously
   - View the run count as it increases

5. **Stop When Done**
   - Click **"⏹ Stop"** to halt execution
   - The window can be minimized while running

---

## 🎯 Use Cases

- **Automated Testing** - Run test scripts continuously
- **Web Scraping** - Execute scraping scripts at regular intervals
- **Data Processing** - Process batches of data repeatedly
- **Monitoring** - Check system status or logs periodically
- **Bot Automation** - Keep automation scripts running
- **Development Testing** - Test scripts during development
- **Scheduled Tasks** - Run tasks at custom intervals

---

## 📝 Example Scripts

### Python Script (Cross-Platform)

**simple-test.py**
```python
#!/usr/bin/env python3
from datetime import datetime

print(f"[{datetime.now().strftime('%H:%M:%S')}] Script running...")
# Your automation code here
print("Done!")
```

### Windows Popup Script

**windows-popup-script.py**
```python
#!/usr/bin/env python3
import tkinter as tk
from tkinter import messagebox
from datetime import datetime

root = tk.Tk()
root.withdraw()
root.attributes('-topmost', True)

timestamp = datetime.now().strftime("%H:%M:%S")
messagebox.showinfo("Macro Bot", f"Script running at {timestamp}")

root.destroy()
```

### macOS Notification Script

**mac-test-script.py**
```python
#!/usr/bin/env python3
import subprocess
from datetime import datetime

timestamp = datetime.now().strftime("%H:%M:%S")
applescript = f'''
display notification "Script running at {timestamp}" with title "Macro Bot"
'''
subprocess.run(["osascript", "-e", applescript])
```

### Windows Batch Script

**example.bat**
```batch
@echo off
echo [%TIME%] Running automation task...
REM Your automation commands here
echo Done!
```

### macOS Shell Script

**example.sh**
```bash
#!/bin/bash
echo "[$(date +%H:%M:%S)] Running automation task..."
# Your automation commands here
echo "Done!"
```

---

## 🖥️ Platform-Specific Notes

### macOS
- Python scripts use `python3` command
- Shell scripts (`.sh`) are supported
- Notifications require no additional setup
- May need to grant permissions for file access

### Windows
- Python scripts use `python` command
- Batch files (`.bat`, `.cmd`) are supported
- Executable files (`.exe`) can be run directly
- Scripts in shared folders (Parallels/VMware) may have issues - copy to `C:\` drive

---

## 🔧 Troubleshooting

### Build Errors

**macOS: "cargo: command not found"**
```bash
source $HOME/.cargo/env
# or restart terminal
```

**Windows: "link.exe not found"**
- Install Visual Studio Build Tools (see installation section)

**Windows: "failed to remove temporary directory"**
- Move project folder to `C:\` drive (not shared folder)
- Shared folders from VMs can cause build issues

### Runtime Errors

**"Permission denied" when running script**
- macOS: `chmod +x your-script.sh`
- Windows: Right-click macro-bot-gui.exe → Run as Administrator

**"python/python3 not found"**
- Install Python 3 from https://python.org/
- Ensure Python is in your system PATH

**Script doesn't run**
- Check the script path is correct
- Verify the script works manually first
- Check console output for error messages

**Window doesn't appear**
- Check if it opened in another workspace/desktop
- Try running from terminal to see errors
- Ensure GUI dependencies are installed

---

## ⚙️ Advanced Usage


### Custom Delay Strategies

- **Quick Testing**: 1-5 seconds
- **Normal Automation**: 10-30 seconds
- **Normal API Rate Limiting**: 60-300 seconds (1-5 minutes)
- **Hourly Tasks**: 3600 seconds (1 hour)

---

## 📊 Technical Details

### Architecture
- **Language**: Rust
- **GUI Framework**: iced (v0.12)
- **File Dialogs**: rfd (native file pickers)
- **Async Runtime**: tokio

### Script Execution
- Scripts run in background processes
- Output is captured but not displayed (runs silently)
- Each script must fully complete before next execution
- Configurable delay between runs prevents system overload

### Supported File Types

| Extension | Platform | Executor |
|-----------|----------|----------|
| `.py` | Both | python3 (Mac), python (Windows) |
| `.sh` | macOS only | sh |
| `.bat`, `.cmd` | Windows only | cmd |
| `.exe` | Windows only | Direct execution |
| Other | Both | Attempts direct execution |

---

## 📄 License

MIT License - Feel free to use, modify, and distribute.

---

## 💡 Tips & Best Practices

1. **Test scripts manually first** before using with Macro Bot
2. **Start with longer delays** (30+ seconds) and decrease as needed
3. **Monitor the first few runs** to ensure scripts work correctly
4. **Use logging in your scripts** to track execution history
5. **Keep scripts simple** - complex scripts may cause issues
6. **Handle errors in scripts** to prevent crashes
7. **Don't run resource-intensive scripts** at very short intervals
8. **Minimize the window** to keep working on other tasks
9. **Use absolute paths** in your scripts for reliability
10. **Save your work** before running unknown scripts

---

## 🆘 Support

For issues, questions, or feature requests:
- Check the Troubleshooting section above
- Review example scripts for reference
- Ensure all requirements are installed
- Verify scripts work when run manually

---

## 🎉 Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [iced](https://github.com/iced-rs/iced) - Cross-platform GUI framework
- [tokio](https://tokio.rs/) - Asynchronous runtime
- [rfd](https://github.com/PolyMeilex/rfd) - Native file dialogs

---

## 📌 Version

**Version 1.0.0**
- Initial release
- macOS and Windows support
- GUI & UI interface
- Configurable delays
- Multiple script types

---

**Made with love❤️ by OtterBot**
