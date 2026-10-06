# Сторонние компоненты и лицензии — Кнорозов PRO

Кнорозов PRO — закрытое (проприетарное) приложение. В нём используются только компоненты под
разрешительными лицензиями (MIT, Apache-2.0, BSD, ISC, Zlib, BSL-1.0, Unicode, OFL) и MPL-2.0
(Symphonia, cssparser/selectors — используются без изменений; исходный код доступен по ссылкам ниже).
**GPL/LGPL-компонентов нет.** ffmpeg не поставляется.

## Основные компоненты

| Компонент | Лицензия | Где используется | Ссылка |
|---|---|---|---|
| Tauri 2 | MIT / Apache-2.0 | оболочка приложения | https://github.com/tauri-apps/tauri |
| whisper.cpp + ggml | MIT | распознавание речи (встроено в исполняемый файл) | https://github.com/ggml-org/whisper.cpp |
| whisper-rs / whisper-rs-sys | Unlicense | привязки Rust к whisper.cpp | https://codeberg.org/tazz4843/whisper-rs |
| Модели Whisper в формате ggml (`ggml-small-q5_1.bin`, `ggml-large-v3-turbo-q5_0.bin`) | MIT (OpenAI Whisper) | скачиваются при первом запуске | https://huggingface.co/ggerganov/whisper.cpp |
| sherpa-onnx | Apache-2.0 | извлечение эмбеддингов спикеров (статически) | https://github.com/k2-fsa/sherpa-onnx |
| sherpa-rs | MIT | привязки Rust к sherpa-onnx | https://github.com/thewh1teagle/sherpa-rs |
| ONNX Runtime | MIT | движок нейросети для sherpa-onnx (статически) | https://github.com/microsoft/onnxruntime |
| 3D-Speaker CAM++ `speech_campplus_sv_zh_en_16k-common_advanced` (ONNX) | Apache-2.0 | модель спикеров, встроена в установщик | https://www.modelscope.cn/models/iic/speech_campplus_sv_zh_en_16k-common_advanced , https://github.com/modelscope/3D-Speaker |
| Symphonia | MPL-2.0 | декодирование mp3/aac/mp4/mov/wav/ogg/flac | https://github.com/pdeljanov/Symphonia |
| libopus (через opusic-sys) | BSD-3-Clause | декодирование Opus (голосовые сообщения) | https://opus-codec.org/license/ |
| symphonia-adapter-libopus | MIT / Apache-2.0 | | https://github.com/aschey/symphonia-adapters |
| Rubato | MIT | ресемплинг в 16 кГц | https://github.com/HEnquist/rubato |
| docx-rs | MIT | экспорт в Word | https://github.com/bokuweb/docx-rs |
| krilla | MIT / Apache-2.0 | экспорт в PDF | https://github.com/LaurenzV/krilla |
| Шрифт Inter 4.1 | SIL Open Font License 1.1 | интерфейс и PDF | https://github.com/rsms/inter |
| Svelte 5 | MIT | интерфейс (скомпилирован в бандл) | https://github.com/sveltejs/svelte |
| @tauri-apps/api и плагины dialog, clipboard-manager, opener | MIT / Apache-2.0 | интерфейс | https://github.com/tauri-apps/plugins-workspace |

Тестовые аудиофрагменты в `tests/` (не входят в приложение): LibriVox «Дубровский» (А. С. Пушкин) и
«The Art of War» — public domain; фрагмент речи Дж. Ф. Кеннеди (`jfk`) из репозитория whisper.cpp — public domain.

## Тексты лицензий

### MIT
Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit
persons to whom the Software is furnished to do so, subject to the following conditions: The above copyright notice
and this permission notice shall be included in all copies or substantial portions of the Software. THE SOFTWARE IS
PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED.

### Apache License 2.0
Полный текст: https://www.apache.org/licenses/LICENSE-2.0

### BSD-3-Clause (libopus)
Copyright 2001–2023 Xiph.Org, Skype Limited, Octasic, Jean-Marc Valin, Timothy B. Terriberry, CSIRO, Gregory Maxwell,
Mark Borgerding, Erik de Castro Lopo. Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met: redistributions of source code must retain the above
copyright notice, this list of conditions and the following disclaimer; redistributions in binary form must reproduce
the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other
materials provided with the distribution; neither the name of Internet Society, IETF or IETF Trust, nor the names of
specific contributors, may be used to endorse or promote products derived from this software without specific prior
written permission. THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS".

### Mozilla Public License 2.0 (Symphonia, cssparser, selectors, option-ext, dtoa-short)
Полный текст: https://mozilla.org/MPL/2.0/ . Исходный код этих библиотек не изменялся и доступен по ссылкам в таблице ниже.

### SIL Open Font License 1.1 (Inter)
Полный текст: https://openfontlicense.org — Copyright (c) 2016 The Inter Project Authors (https://github.com/rsms/inter).
