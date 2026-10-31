Unicode True

Name "GENTWO"
OutFile "GENTWO-Setup.exe"
InstallDir "$LOCALAPPDATA\Programs\GENTWO"
RequestExecutionLevel user
Icon "../../assets/gentwo_installer.ico"
UninstallIcon "../../assets/gentwo_installer.ico"
Section "GENTWO"
    SetOutPath "$INSTDIR"
    File /r "../../*.*"
    WriteUninstaller "$INSTDIR/unins000.exe"
SectionEnd
Section "Uninstal"
	Delete /r "$INSTDIR/"
SectionEnd