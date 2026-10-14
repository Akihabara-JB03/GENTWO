@echo off
git add . || exit /b 1
git commit -m "%~1" || exit /b 1
git push || exit /b 1
pause