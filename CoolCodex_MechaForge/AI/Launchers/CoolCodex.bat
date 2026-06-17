@echo off
setlocal

rem SessionRoot is manually configured and independent from LauncherDir.
rem It is the directory Codex enters before launch.
set "SESSION_ROOT_TARGET=C:\CodexLab\codex\工作区1\AI\SessionRoots\CoolCodex"

rem LauncherDir = this BAT file's directory.
set "LAUNCHER_DIR=%~dp0"
set "COOL_SYSTEM_DIR=%LAUNCHER_DIR%.cool-system"

rem Compatibility env for current/old code paths.
rem Future code should prefer COOL_SYSTEM_DIR and derive config/scope/command from it.
set "COOL_SYSTEM_CONFIG=%COOL_SYSTEM_DIR%\config.toml"

rem Prefer the isolated release1 build; fall back to the normal release build.
set "CODEX_REPO=C:\CodexLab\codex"
set "CODEX_EXE=%CODEX_REPO%\codex-rs\target\release1\release\codex.exe"
if not exist "%CODEX_EXE%" set "CODEX_EXE=%CODEX_REPO%\codex-rs\target\release\codex.exe"

cd /d "%SESSION_ROOT_TARGET%"
if errorlevel 1 (
  echo Failed to enter SessionRoot: %SESSION_ROOT_TARGET%
  pause
  exit /b 1
)
set "SESSION_ROOT=%CD%"
set "COOL_DIR=%SESSION_ROOT%\.cool"

echo ================================
echo Before launch: related processes
echo ================================
tasklist | findstr /i "codex app-server exec-server"
if errorlevel 1 echo No related process before launch.

echo.
echo ================================
echo Cool paths
echo ================================
echo LauncherDir=%LAUNCHER_DIR%
echo CoolSystemDir=%COOL_SYSTEM_DIR%
echo SessionRoot=%SESSION_ROOT%
echo CoolDir=%COOL_DIR%
echo COOL_SYSTEM_CONFIG=%COOL_SYSTEM_CONFIG%
echo CODEX_EXE=%CODEX_EXE%

echo.
echo ================================
echo Launching CoolCodex
echo SafeMode should default ON.
echo PermissionProfile should default CoolReadWrite.
echo ================================
echo.

if not exist "%CODEX_EXE%" (
  echo Codex executable not found: %CODEX_EXE%
  echo Build it first, for example:
  echo   cd /d %CODEX_REPO%\codex-rs
  echo   set "CARGO_TARGET_DIR=target\release1" ^&^& cargo build --release --bin codex
  pause
  exit /b 1
)

"%CODEX_EXE%"

echo.
echo ================================
echo Codex exited. Waiting 3 seconds...
echo ================================
timeout /t 3 /nobreak >nul

echo.
echo ================================
echo After exit: related processes
echo ================================
tasklist | findstr /i "codex app-server exec-server"
if errorlevel 1 echo No related process after exit.

echo.
echo Test finished.
pause
