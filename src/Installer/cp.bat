@echo off

robocopy "../../" "../dist/" /E /XD ".git" "../dist"
pause