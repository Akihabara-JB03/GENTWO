Get-ChildItem ".." -Exclude "dist","Installer" |
    Copy-Item -Destination "..\dist" -Recurse -Force