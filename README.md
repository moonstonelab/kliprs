# kliprs

## Dependecies

On Debian based distros:

```bash
sudo apt-get install xorg-dev
sudo apt-get install libxcb-xfixes0-dev
```

## How to build SQLite3 .lib file on Windows

1. Download amalgamation source from [https://www.sqlite.org/download.html](https://www.sqlite.org/download.html)
2. Download precompiled `sqlite-dll-win-x64-xxxxxxxx.zip` binary from the same page
3. Extract both archives to the same directory
4. Open Developer Command Prompt for Visual Studio 20xx: [https://learn.microsoft.com/en-us/visualstudio/ide/reference/command-prompt-powershell?view=vs-2022](https://learn.microsoft.com/en-us/visualstudio/ide/reference/command-prompt-powershell?view=vs-2022)
5. Go to directory where you've extracted source code and binary files (via opened cmd)
6. Run:

```powershell
lib /DEF:sqlite3.def /OUT:sqlite3.lib /MACHINE:x64
```

## How to compile rust application with sqlite3.lib on Windows.

```powershell
$Env:SQLITE3_LIB_DIR = "<path-to-folder-where-sqlite3-dot-lib-lives>"
cargo run -p kliprs-api
```
