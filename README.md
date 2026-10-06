# Кнорозов PRO

Офлайн-приложение для расшифровки аудио и видео в текст — для **Windows** и **macOS**.
Названо в честь Юрия Кнорозова, расшифровавшего письменность майя.

![Дизайн](docs/design-reference.png)

## Что умеет (v0.1)

* Перетащите файл или нажмите «Выбрать файл»: **mp3, m4a, wav, ogg/opus (голосовые), mp4, mov** (+ flac, aac).
* Язык — **автоопределение** или выбор из 100 языков Whisper (русский и английский — первыми).
* Качество: **«Быстро»** (Whisper small, 190 МБ) или **«Точно»** (Whisper large-v3-turbo, 574 МБ).
* **«Разделить по спикерам»** — автоматическая разметка «Спикер 1, Спикер 2…», имя меняется сразу во всём тексте.
* Живой текст во время расшифровки, прогресс и оценка оставшегося времени, «Отмена».
* Редактор: поиск, правка текста, кликабельные таймкоды, плеер (скорость, громкость), подсветка текущего абзаца.
* Экспорт: **Word (.docx), PDF, TXT, SRT** и «Копировать всё».
* «Недавние» хранятся только на компьютере. Светлая и тёмная тема.
* **Работает без интернета.** Интернет нужен один раз — скачать модели (можно указать зеркало).

## Установка

См. [docs/install-ru.md](docs/install-ru.md). Сборки: GitHub Actions → последний успешный запуск **CI** →
раздел *Artifacts* (`Knorozov-PRO-windows-x64`, `Knorozov-PRO-macos-arm64`).

## Для разработчика

```bash
pnpm install
pnpm tauri dev          # запуск в режиме разработки
pnpm check && pnpm lint # проверка фронтенда
cargo test -p knorozov-core            # юнит-тесты (экспорт, SRT, спикеры…)
KNOROZOV_TEST_MODEL=/path/ggml-tiny.bin cargo test --release -p knorozov-core   # + распознавание RU/EN
cargo run --release -p knorozov-cli -- transcribe model.bin audio.mp3   # CLI для замеров
```

Документы: [архитектура](docs/architecture.md) · [замеры скорости](docs/benchmarks.md) ·
[лицензии](THIRD_PARTY_LICENSES.md).

Брендинг (иконка, иллюстрации) — `src-tauri/app-icon.png` (→ `pnpm tauri icon src-tauri/app-icon.png -o src-tauri/icons`),
`src/assets/empty-state.png`, `src/assets/first-launch.png`, `src/assets/logo.png`.
