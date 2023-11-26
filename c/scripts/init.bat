@echo off

REM Compile the C program along with noc.c from the /lib directory
gcc -NEC_INFO/lib main.c ./lib/noc.c ./lib/smd.c -o smd.exe

REM Check if compilation was successful
IF %ERRORLEVEL% NEQ 0 (
    echo Compilation failed.
    exit /b 1
)


REM Assuming smd.exe is in the current directory
setx /M PATH "%PATH%;%CD%"

REM Verify if the update to PATH was successful
IF %ERRORLEVEL% EQU 0 (
    echo Installation successful. You can run the program using 'smd' command.
) ELSE (
    echo Installation failed.
)