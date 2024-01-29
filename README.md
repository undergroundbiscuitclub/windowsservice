Quick Windows Service example that simply writes to a file (c:\PENTEST_WHOAMI.txt) with the whoami output ran in the service.

This is a PoC to demostrate that a service is running as a certain user.

Can be used for Unquoted Service Paths or Permissive ACL

Build with rust on any platform (requires mingw-w64):
```
cargo update
cargo build --target x86_64-pc-windows-gnu --release
```

Note if you wish to change the icon, change `the assets/app.ico`. You will need to recompile on a windows machine in order for the icon to compile.