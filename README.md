# 🚀 Rust + Vue.js Fullstack Template

Современный шаблон для быстрого старта fullstack-проектов с использованием:
- **Rust** (бэкенд на Axum)
- **Vue.js** (фронтенд на Vite)
- **Caddy** (прокси-сервер)
- **Docker** (оркестрация)

[![Docker](https://img.shields.io/badge/Docker-✓-blue?logo=docker)](https://docker.com)
[![Rust](https://img.shields.io/badge/Rust-✓-orange?logo=rust)](https://rust-lang.org)
[![Vue.js](https://img.shields.io/badge/Vue.js-✓-brightgreen?logo=vue.js)](https://vuejs.org)

![Architecture Diagram](https://via.placeholder.com/800x400.png?text=Project+Architecture) <!-- Замените на реальную диаграмму -->

## 🌟 Особенности

- 🐳 Полная Docker-ориентированная разработка (Compose)
- 🔄 Hot-reload для фронтенда и бэкенда
- 📦 Готовые окружения: development & production
- 🔒 Автоматический HTTPS через Caddy
- 🗄️ Поддержка MySQL с health-check
- 🛠️ Оптимизированные многоступенчатые Docker-образы

## 🛠 Стек технологий

**Бэкенд**
[![Rust](https://img.shields.io/badge/-Rust-000000?logo=rust)](https://rust-lang.org)
[![Axum](https://img.shields.io/badge/-Axum-FF7139)](https://github.com/tokio-rs/axum)
[![MySQL](https://img.shields.io/badge/-MySQL-4479A1?logo=mysql)](https://mysql.com)

**Фронтенд**
[![Vue.js](https://img.shields.io/badge/-Vue.js-4FC08D?logo=vue.js)](https://vuejs.org)
[![Vite](https://img.shields.io/badge/-Vite-646CFF?logo=vite)](https://vitejs.dev)
[![TypeScript](https://img.shields.io/badge/-TypeScript-3178C6?logo=typescript)](https://typescriptlang.org)

**Инфраструктура**
[![Docker](https://img.shields.io/badge/-Docker-2496ED?logo=docker)](https://docker.com)
[![Caddy](https://img.shields.io/badge/-Caddy-1DAB68)](https://caddyserver.com)

## 🚀 Быстрый старт

### Предварительные требования
- Docker >= 20.10
- Docker Compose >= 2.0
- Node.js >= 18 (опционально)

```bash
# 1. Клонировать репозиторий
git clone https://github.com/yourusername/rust-vue-template.git
cd rust-vue-template

# 2. Создать файл окружения
cp .env.example .env

# 3. Запустить в режиме разработки
docker-compose -f docker-compose.dev.yaml up --build

# Откройте в браузере
🌐 Frontend: http://localhost
🔌 API: http://localhost/api
