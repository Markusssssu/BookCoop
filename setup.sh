#!/bin/bash

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${GREEN}[1/3] Проверка Docker...${NC}"
if ! docker info > /dev/null 2>&1; then
    echo -e "${RED}ОШИБКА: Docker не запущен или не установлен.${NC}"
    exit 1
fi

echo -e "${GREEN}[2/3] Остановка активных контейнеров...${NC}"
docker-compose down

echo -e "${GREEN}[3/3] Сборка и запуск проекта...${NC}"
echo -e "Фронтенд: ${GREEN}http://localhost:3000${NC}"
echo -e "Бэкенд:   ${GREEN}http://localhost:8080${NC}"

docker-compose up --build
