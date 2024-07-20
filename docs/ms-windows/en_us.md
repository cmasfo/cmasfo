# Microsoft Windows

Windows (or MS Windows) is an [operating system](../system-operating/README.md), [developed](../systems-development/README.md) by [Microsoft](../ms%20(company)/README.md). It is the most popular desktop operating system, dominating the majority of the desktop OS market.

Windows stands out as a representative non-Unix operating system, contrasting with Unix-like systems such as Linux, macOS, and BSD. Due to its non-Unix architecture, Windows exhibits several fundamental differences:

- **Backslash as Path Separator**
  - While Unix-like systems use a forward-slash (/) as the path separator, Windows uses a backslash (\\). This choice can be problematic in programming languages like C, where two backslashes are needed to represent one backslash in string literals. As a result, directly copying and pasting Windows paths into string literals can cause issues.
- **CRLF as End of Line**
  - One of the most significant critiques of Windows is its use of CRLF (Carriage Return and Line Feed) to denote the end of a line, originating from typewriter conventions. Although modern systems typically use LF (Line Feed) alone, Windows persists with CRLF. This difference can cause frequent problems for developers, leading to frustration.
- **Environment Variable Syntax (`%ENV_VAR%`)**
  - In Unix-like systems, environment variables are denoted with `$ENV_VAR`, while Windows uses `%ENV_VAR%`. Although this difference is less problematic, the `%` sign is also used for string formatting in languages like C (e.g., `printf("%d", num);`). This can necessitate escaping the `%` symbol (`%%`) in certain contexts.
- **Non-Root File System**
  - Unix-like systems have a root (`/`) in their file system hierarchy, whereas Windows starts with drive letters like `C:` or `D:`. This difference can complicate the portability of Unix scripts and programs to Windows. To mitigate this, Microsoft introduced the Windows Subsystem for Linux (WSL), but it can still be slow for manipulating Windows file systems, and differences like LF/CRLF remain problematic. As a result, developing cross-platform programs that work seamlessly on both Windows and Unix systems can be challenging.

Windows remains a popular desktop OS, with its roots in MS-DOS. For those interested in understanding Windows better, exploring MS-DOS can provide valuable insights.
