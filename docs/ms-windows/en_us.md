
# Microsoft Windows

Windows(or MS Windows) is an [operating system](../system-operating/README.md) that is [developed](../systems-development/README.md) by [Microsoft](../ms%20(company)/README.md).

MS Windows is the most popular desktop operating system, dominating most of the desktop OS market. 

Windows is also the most representative non-Unix operaring system, while other operating systems like Linux, macOS, BSD are regarded as Unix-like operating systems.

Since it's non-Unix system, there are some differences from the fundamental level.

- Backslash as Path Separator
	- Unix-like systems use forward-slash(/) as path separator. And Windows uses backslash(\\).
	- This seems quite an unreasonable choice, since in programming languages like C requires two backslashes to represent one backslash in string literals.
	- This means the direct copy-paste of Windows path won't work immediately in those string literals.
	- It is understandable for that non-Unix system has different system architecture, but some Windows' choices like this seem quite undefendable.
- CRLF as End of Line
	- This is one of the biggest critiques against MS Windows, just like backslash path separator that is explained above.
	- CRLF itself is orginated from the typewriter. At that time, CR(carriage return) meant going at the start of the line, and LF(line feed) meant going to the next line, while the cursor position remained same. So to start fresh at the next line, it was necessary to do CRLF, not only LF.
	- But for computers, LF is enough to represent the fresh start of next line. However, Windows is using CRLF as the default. It would be understandable if LF meant just going down with same cursor position, but even for Windows, LF still means new line, just same as CRLF.
	- Backslash, CRLF, these subtle differences might not sound big deal at first time, but what matters is the frequency. Developers might face these problems so often, and it might make them quite irritated.
- Environmemt Variable Syntax (`%ENV_VAR%`)
	- In Unix-like systems, the usage of environmemt variables is like `$ENV_VAR`. But Windows is `%ENV_VAR%`.
	- This might seem less unreasonable. But in C literals, `%` sign can be used for string formatting.
	- e.g. `printf("%d", num);`
	- Just like double backslash(`//`), the way to represent one `%` symbol is `%%`.
- Non-Root File System
	- Unix-like system has the concept of the root(`/`) in the file system hierarchy. But Windows doesn't. It needs to start with drive, like `C:` drive or `D:` drive.
	- This might belong to understandable differences unlike others. But yet, the absence of the root and the difference of file system architecture are quite big deals. It literally make Unix scripts or programs not portable.
	- To reduce those problems, Windows made WSL(Windows Subsystem for Linux). But still, WSL is slow for manipulating Windows file system, and the differences like LF/CRLF still cause errors. So it might feels like that it is just a separate virtual machine. It is also not so convenient to use WSL files from Windows-side.
	- In conclusion, it is hard to develop a program which is usable for Windows and Unix both. And in many cases, developers choose Windows, since it is already popular with things like GUI. And maybe this was what Microsoft wanted.

Windows is a popular desktop OS, but the most fundamental things are originated from MS-DOS. So if you are interested in Windows system, try investigating about MS-DOS.
