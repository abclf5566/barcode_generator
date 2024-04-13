@echo off
for /f "tokens=4" %%i in ('netsh mbn show interface ^| findstr /R "[0-9][0-9]*$"') do @echo %%i
