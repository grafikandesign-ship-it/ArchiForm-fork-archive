@echo off
setlocal
cd /d "%~dp0"
python --version >nul 2>nul
if errorlevel 1 (
  echo Python 3 non trovato nel PATH. Installa Python 3 oppure avvia tools\dae-converter\index.html manualmente.
  pause
  exit /b 1
)
start "ArchiForm DAE Converter server" python -m http.server 8765 --bind 127.0.0.1 --directory "%~dp0tools\dae-converter"
echo Avvio del server locale...
powershell -NoProfile -Command "$end=(Get-Date).AddSeconds(15); do { try { $c=[Net.Sockets.TcpClient]::new(); $c.Connect('127.0.0.1',8765); $c.Dispose(); exit 0 } catch { Start-Sleep -Milliseconds 300 } } while ((Get-Date) -lt $end); exit 1"
if errorlevel 1 (
  echo Il server non si e avviato. Controlla la finestra del server: la porta 8765 potrebbe essere gia occupata.
  pause
  exit /b 1
)
start "" http://127.0.0.1:8765
echo La web app e aperta nel browser. Per arrestarla, chiudi la finestra del server.
