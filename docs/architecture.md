# Кнорозов PRO — архитектура (v1 / MVP)

Офлайн-приложение для расшифровки аудио и видео в текст для Windows и macOS.
Всё работает локально; в сеть приложение ходит только чтобы **один раз скачать модели**
распознавания (Hugging Face или зеркало, настраивается).

## 1. Стек

| Слой | Выбор | Лицензия | Комментарий |
|---|---|---|---|
| Оболочка | **Tauri v2** (Rust + WebView) | MIT/Apache-2.0 | WebView2 на Windows, WKWebView на macOS |
| Фронтенд | **Svelte 5 + Vite + TypeScript**, собственный CSS (CSS-переменные, светлая/тёмная тема) | MIT | Tailwind сознательно не используем — см. «Отклонения» |
| Распознавание | **whisper.cpp** через `whisper-rs` 0.16 | MIT (whisper.cpp, whisper-rs) | Metal на macOS, CPU (AVX2/NEON) на Windows |
| Декодирование | **symphonia** 0.6 (mp3, aac/m4a, mp4/mov, wav, ogg/vorbis, flac) + **libopus** через `symphonia-adapter-libopus` (ogg/opus — голосовые) | MPL-2.0* / BSD-3 (libopus) / MIT/Apache | *symphonia — MPL-2.0 (file-level copyleft, совместима с закрытым кодом при неизменённых исходниках). ffmpeg НЕ поставляется |
| Ресемплинг | `rubato` (sinc, FFT) → 16 кГц mono f32 | MIT | |
| Диаризация | **sherpa-onnx** (`sherpa-rs`, статическая сборка) — только *speaker embedding* (3D-Speaker CAM++) + собственная агломеративная кластеризация по сегментам Whisper | Apache-2.0 (sherpa-onnx, 3D-Speaker), MIT (onnxruntime, sherpa-rs) | без pyannote — см. §5 |
| DOCX | `docx-rs` | MIT | |
| PDF | `krilla` + шрифт **Inter** (встраивается, кириллица) | MIT/Apache, OFL-1.1 | |
| Загрузка моделей | `ureq` (rustls) + `sha2` | MIT/Apache | докачка (HTTP Range), проверка SHA-256 |
| Хранилище | JSON-файлы в app data dir | — | «Недавние», расшифровки, настройки |

Только MIT/Apache/BSD/OFL (+ MPL-2.0 для symphonia, которая не требует раскрытия нашего кода).
GPL/LGPL-компонентов в v1 нет. Полный список — `THIRD_PARTY_LICENSES.md` и экран «О программе».

## 2. Структура репозитория

```
knorozov-pro/
├─ Cargo.toml                 # workspace
├─ crates/
│  ├─ core/                   # knorozov-core: вся логика без GUI (тестируется в CI)
│  │  ├─ src/audio.rs         # декодирование любых форматов → 16 кГц mono f32, probe длительности
│  │  ├─ src/asr.rs           # обёртка над whisper-rs: прогресс, потоковые сегменты, отмена
│  │  ├─ src/models.rs        # каталог моделей (HF репо, размер, SHA-256), загрузка с докачкой
│  │  ├─ src/diarize.rs       # эмбеддинги спикеров (sherpa-onnx) + кластеризация (feature `diarization`)
│  │  ├─ src/transcript.rs    # модель данных: сегменты → абзацы, спикеры, переименование
│  │  ├─ src/export/          # docx.rs, pdf.rs, txt.rs, srt.rs
│  │  ├─ src/languages.rs     # 99 языков Whisper с русскими названиями
│  │  └─ assets/fonts/        # Inter (OFL) для PDF
│  └─ cli/                    # knorozov-cli: CLI-харнесс для бенчмарков и отладки
├─ src-tauri/                 # Tauri-приложение: команды, состояние, хранилище, события
│  ├─ icons/                  # иконки (генерируются `tauri icon` из app-icon.png)
│  └─ resources/models/       # встроенная модель эмбеддингов спикеров (~28 МБ)
├─ src/                       # Svelte-фронтенд
│  ├─ screens/                # Main, Processing, Editor, FirstRun, Settings, About
│  ├─ lib/                    # api.ts (invoke/events), store, форматирование времени
│  └─ assets/                 # иллюстрации, шрифт Inter
├─ tests/fixtures/            # короткие RU/EN образцы (public domain)
├─ docs/                      # architecture.md, benchmarks.md, install-ru.md, design-reference.png
└─ .github/workflows/         # CI: тесты + сборка .exe/.dmg/.pkg
```

## 3. Поток данных

```
Файл (drag&drop / «Выбрать файл»)
   │  invoke probe_media → {формат, длительность}
   ▼
start_transcription(path, язык, качество, спикеры)        [фоновый поток Rust]
   1. audio::decode_to_16k_mono  — symphonia/libopus → rubato        (0–5 %)
   2. asr::transcribe            — whisper.cpp, callbacks:            (5–95 %)
        progress → событие `job://progress` {percent, eta_sec}
        new segment → событие `job://segment` (живой текст на экране «Обработка»)
        abort ← флаг «Отмена»
   3. diarize::assign_speakers   — эмбеддинг на каждый сегмент,       (95–100 %)
        агломеративная кластеризация по косинусному сходству
   4. transcript::build          — сегменты → абзацы (смена спикера / пауза > 2 с / длина)
   5. storage::save_project      — JSON в app data/projects/<id>.json, обновление «Недавних»
   ▼
событие `job://done` {projectId} → экран «Результат»
   • редактирование текста абзаца, переименование спикера (во всём документе)
   • таймкод → плеер <audio> (asset protocol; для ogg/opus — WAV-копия в кэше)
   • Экспорт: DOCX / PDF / TXT / SRT (Rust) → диалог «Сохранить»; «Копировать всё» → буфер обмена
```

## 4. Модели распознавания (whisper.cpp ggml)

Репозиторий: [`ggerganov/whisper.cpp`](https://huggingface.co/ggerganov/whisper.cpp) на Hugging Face (MIT; веса Whisper — MIT, OpenAI).
URL: `{mirror}/ggerganov/whisper.cpp/resolve/main/{file}`, по умолчанию `mirror = https://huggingface.co`,
в настройках можно указать зеркало (например `https://hf-mirror.com`) — полезно, если HF недоступен из РФ.

| Режим | Файл | Размер | SHA-256 |
|---|---|---|---|
| **Быстро** | `ggml-small-q5_1.bin` | 190 МБ | `ae85e4a935d7a567bd102fe55afc16bb595bdb618e11b2fc7591bc08120411bb` |
| **Точно** | `ggml-large-v3-turbo-q5_0.bin` | 574 МБ | `394221709cd5ad1f40c46e6031ca61bce88931e6e088c188294c6d5a55ffa7e2` |
| CI/тесты | `ggml-tiny.bin` | 78 МБ | `be07e048e1e599ad46341c8d2a135645097a538221678b7acdd1b1919c6e1b21` |

При первом запуске скачиваются обе модели (~764 МБ). Загрузка: во временный файл `*.part`,
докачка через `Range`, проверка SHA-256, атомарное переименование. Отмена — в любой момент.

Почему так: `small` заметно лучше `base` на русском, а q5_1 почти не теряет качество; `large-v3-turbo`
(4 слоя декодера) — лучшее соотношение качество/скорость среди больших моделей, q5_0 — ~3× меньше f16.

### Модель эмбеддингов спикеров
`3dspeaker_speech_campplus_sv_zh_en_16k-common_advanced.onnx` (28 МБ), 3D-Speaker CAM++ (ModelScope
`iic/speech_campplus_sv_zh_en_16k-common_advanced`, **Apache-2.0**), ONNX-версия из релизов
`k2-fsa/sherpa-onnx` (тег `speaker-recongition-models`). **Встроена в установщик**, чтобы не зависеть от GitHub при первом запуске.

## 5. Диаризация («Разделить по спикерам»)

Полноценный пайплайн sherpa-onnx использует сегментацию pyannote-segmentation-3.0 (gated на HF) или
Reverb (некоммерческая лицензия) — оба исключены брифом. Поэтому v1 делает так:

1. Сегменты Whisper (обычно 2–10 с) служат «окнами речи».
2. Для каждого сегмента ≥ 0,6 с считается эмбеддинг CAM++ (sherpa-onnx `SpeakerEmbeddingExtractor`).
3. Агломеративная кластеризация (average linkage, косинусное сходство, порог ~0,55) → `Спикер 1..N`.
   Короткие сегменты получают спикера соседнего.

Ограничения (best-effort): смена спикера внутри одного сегмента Whisper не разделяется; перебивания
и одновременная речь размечаются неточно. Опция отключаемая; при ошибке диаризации расшифровка всё равно сохраняется.

## 6. Входные форматы

mp3, m4a (AAC-LC), wav, ogg (Vorbis/Opus — голосовые Telegram/WhatsApp), mp4/mov (дорожка AAC), flac.
Не поддерживаются в v1: HE-AAC, AC-3/E-AC-3 в видео, webm. Если декодер не справился и в системе есть `ffmpeg`
в `PATH`, используется он (мы его **не поставляем**).

## 7. Упаковка и CI

GitHub Actions (`.github/workflows/ci.yml`):
* `ubuntu-latest`: юнит-тесты core, `svelte-check`, ESLint (дёшево).
* `windows-latest`: тесты core + интеграционный тест (tiny, RU+EN) + `tauri build` → NSIS `.exe`.
* `macos-latest` (Apple Silicon): то же + `.dmg`; `.pkg` собирается `pkgbuild` из `.app`.
Без подписи (см. `docs/install-ru.md`). Артефакты — только workflow artifacts (приватный репозиторий).

## 8. Отклонения от брифа

* **Без Tailwind**: интерфейс небольшой, собственный CSS с переменными проще для тем и не тянет
  сборочных зависимостей.
* **Диаризация без модели сегментации** (см. §5) — чтобы остаться в рамках Apache/MIT и не использовать gated pyannote.
* **libopus (BSD-3)** дополнительно к symphonia — symphonia не декодирует Opus, а голосовые сообщения это Opus.
* **symphonia — MPL-2.0** (не MIT/Apache/BSD): слабый file-level copyleft, разрешает закрытое приложение;
  мы не модифицируем её исходники. Альтернатива (ffmpeg, LGPL) хуже.
* **Intel Mac**: основная цель — Apple Silicon; universal-сборка — если получится в CI (см. отчёт).

## 9. Риски

| Риск | Митигация |
|---|---|
| HF недоступен/медленный из РФ | зеркало в настройках, докачка, понятная ошибка |
| Медленно на слабых Windows-ПК (только CPU) | режим «Быстро», честная оценка оставшегося времени; позже Vulkan/CUDA |
| Неподписанные сборки пугают пользователей | инструкция в `docs/install-ru.md`; позже — подпись/нотаризация |
| Качество диаризации | best-effort, отключаемо, ограничения описаны |
| Видео с нестандартными кодеками | понятная ошибка; fallback на системный ffmpeg |
| Длинные файлы (часы) | потоковое декодирование, ~230 МБ RAM на час аудио; whisper.cpp обрабатывает окнами по 30 с |
| Лимит минут GitHub Actions (бесплатный план, macOS ×10) | кэш сборки, объединённые джобы, ручной запуск сборки установщиков |
