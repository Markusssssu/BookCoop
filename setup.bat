@echo off
echo [1/3] Checking Docker status...
docker info >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Docker is not running. Please start Docker Desktop.
    pause
    exit /b
)

echo [2/3] Cleaning up old containers (if any)...
docker-compose down

echo [3/3] Building and starting the project...
echo Frontend will be available at: http://localhost:3000
echo Backend will be available at:  http://localhost:8080
docker-compose up --build

pause
