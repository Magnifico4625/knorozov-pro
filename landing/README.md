# Лендинг Кнорозов PRO

Статическая страница для GitHub Pages. Она использует Vite из зависимостей основного
проекта; дополнительные зависимости и отдельный lock-файл не нужны.

## Локальный запуск

Из корня репозитория:

```bash
pnpm install --frozen-lockfile
pnpm exec vite --config landing/vite.config.mjs
```

Откройте `http://localhost:4173/knorozov-pro/`.

```bash
pnpm exec vite build --config landing/vite.config.mjs
pnpm exec vite preview --config landing/vite.config.mjs
```

Готовые файлы находятся в `landing/dist/`. Базовый путь `/knorozov-pro/` настроен
для этого репозитория.

## GitHub Pages

1. В **Settings → Pages → Build and deployment** выберите **Source: GitHub Actions**.
2. В **Actions → Landing page → Run workflow** выберите ветку `main`.
3. После успешного задания **Publish GitHub Pages** адрес страницы будет
   `https://magnifico4625.github.io/knorozov-pro/`.

Workflow также запускается при изменении страницы в `main` и при публикации Release.
При любом запуске используется страница из `main`, даже если тег выпуска был создан раньше.
Если задания не стартуют из-за блокировки billing, сначала требуется снять блокировку
аккаунта GitHub. Добавление платёжных реквизитов само по себе может её не снять.

## Установщики

Перед сборкой workflow записывает сведения об опубликованных установщиках в
`public/releases.json`. Страница дополнительно проверяет публичный API GitHub;
сохранённые при сборке ссылки остаются доступны при временной ошибке API.

Черновики, prerelease и выпуски без пары EXE/DMG не показываются как готовые.
До первого опубликованного выпуска кнопки ведут к разделу скачивания или списку Releases.
Одна ARM-сборка macOS покрывает M1, M2, M3 и M4; ссылка на PKG появляется при его наличии.

## Изображения и лицензии

- `alpine-lake.webp` — созданная для этой страницы иллюстрация горного озера.
- `app-preview.png` — превью из `docs/design-reference.png`; на странице видны первые два экрана.
- `logo.png` — действующий логотип приложения из `src/assets/logo.png`.
- Inter — шрифт приложения, лицензия в `public/assets/OFL-Inter.txt`.
- Значки Phosphor — лицензия MIT в `public/assets/icons/LICENSE.txt`.

Отчёт визуальной и браузерной проверки находится в [design-qa.md](../design-qa.md).
