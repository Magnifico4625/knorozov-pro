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


## Rust crates (автоматически, `scripts/gen-licenses.py`)

| Crate | Version | License |
|---|---|---|
| [adler2](https://github.com/oyvindln/adler2) | 2.0.1 | 0BSD OR MIT OR Apache-2.0 |
| [adobe-cmap-parser](https://github.com/jrmuizel/adobe-cmap-parser) | 0.4.1 | MIT |
| [aes](https://github.com/RustCrypto/block-ciphers) | 0.8.4 | MIT OR Apache-2.0 |
| [aho-corasick](https://github.com/BurntSushi/aho-corasick) | 1.1.5 | Unlicense OR MIT |
| [alloc-no-stdlib](https://github.com/dropbox/rust-alloc-no-stdlib) | 3.0.0 | BSD-3-Clause |
| [alloc-stdlib](https://github.com/dropbox/rust-alloc-no-stdlib) | 0.3.0 | BSD-3-Clause |
| [anyhow](https://github.com/dtolnay/anyhow) | 1.0.104 | MIT OR Apache-2.0 |
| [arboard](https://github.com/1Password/arboard) | 3.6.1 | MIT OR Apache-2.0 |
| [arrayref](https://github.com/droundy/arrayref) | 0.3.9 | BSD-2-Clause |
| [arrayvec](https://github.com/bluss/arrayvec) | 0.7.8 | MIT OR Apache-2.0 |
| [autocfg](https://github.com/cuviper/autocfg) | 1.5.1 | Apache-2.0 OR MIT |
| [base64](https://github.com/marshallpierce/rust-base64) | 0.21.7 | MIT OR Apache-2.0 |
| [base64](https://github.com/marshallpierce/rust-base64) | 0.22.1 | MIT OR Apache-2.0 |
| [base64](https://github.com/marshallpierce/rust-base64) | 0.23.1 | MIT OR Apache-2.0 |
| [bindgen](https://github.com/rust-lang/rust-bindgen) | 0.69.5 | BSD-3-Clause |
| [bindgen](https://github.com/rust-lang/rust-bindgen) | 0.72.1 | BSD-3-Clause |
| [bit-set](https://github.com/contain-rs/bit-set) | 0.8.0 | Apache-2.0 OR MIT |
| [bit-vec](https://github.com/contain-rs/bit-vec) | 0.8.0 | Apache-2.0 OR MIT |
| [bitflags](https://github.com/bitflags/bitflags) | 1.3.2 | MIT/Apache-2.0 |
| [bitflags](https://github.com/bitflags/bitflags) | 2.13.2 | MIT OR Apache-2.0 |
| [block-buffer](https://github.com/RustCrypto/utils) | 0.10.4 | MIT OR Apache-2.0 |
| [block-padding](https://github.com/RustCrypto/utils) | 0.3.3 | MIT OR Apache-2.0 |
| [block2](https://github.com/madsmtm/objc2) | 0.6.2 | MIT |
| [brotli](https://github.com/dropbox/rust-brotli) | 9.0.0 | BSD-3-Clause AND MIT |
| [brotli-decompressor](https://github.com/dropbox/rust-brotli-decompressor) | 6.0.1 | BSD-3-Clause/MIT |
| [bs58](https://github.com/Nullus157/bs58-rs) | 0.5.1 | MIT/Apache-2.0 |
| [bumpalo](https://github.com/fitzgen/bumpalo) | 3.20.3 | MIT OR Apache-2.0 |
| [bytemuck](https://github.com/Lokathor/bytemuck) | 1.25.2 | Zlib OR Apache-2.0 OR MIT |
| [bytemuck_derive](https://github.com/Lokathor/bytemuck) | 1.12.1 | Zlib OR Apache-2.0 OR MIT |
| [byteorder](https://github.com/BurntSushi/byteorder) | 1.5.0 | Unlicense OR MIT |
| [byteorder-lite](https://github.com/image-rs/byteorder-lite) | 0.1.0 | Unlicense OR MIT |
| [bytes](https://github.com/tokio-rs/bytes) | 1.12.1 | MIT |
| [bzip2](https://github.com/alexcrichton/bzip2-rs) | 0.4.4 | MIT/Apache-2.0 |
| [bzip2-sys](https://github.com/alexcrichton/bzip2-rs) | 0.1.13+1.0.8 | MIT/Apache-2.0 |
| [camino](https://github.com/camino-rs/camino) | 1.2.6 | MIT OR Apache-2.0 |
| [cargo-platform](https://github.com/rust-lang/cargo) | 0.1.9 | MIT OR Apache-2.0 |
| [cargo_metadata](https://github.com/oli-obk/cargo_metadata) | 0.19.2 | MIT |
| [cargo_toml](https://gitlab.com/lib.rs/cargo_toml) | 1.0.1 | Apache-2.0 OR MIT |
| [cbc](https://github.com/RustCrypto/block-modes) | 0.1.2 | MIT OR Apache-2.0 |
| [cc](https://github.com/rust-lang/cc-rs) | 1.6.0 | MIT OR Apache-2.0 |
| [cexpr](https://github.com/jethrogb/rust-cexpr) | 0.6.0 | Apache-2.0/MIT |
| [cfb](https://github.com/mdsteele/rust-cfb) | 0.14.0 | MIT |
| [cff-parser](https://github.com/jrmuizel/cff-parser) | 0.2.0 | MIT OR Apache-2.0 |
| [cfg-if](https://github.com/rust-lang/cfg-if) | 1.0.5 | MIT OR Apache-2.0 |
| [chacha20](https://github.com/RustCrypto/stream-ciphers) | 0.10.2 | MIT OR Apache-2.0 |
| [chrono](https://github.com/chronotope/chrono) | 0.4.45 | MIT OR Apache-2.0 |
| [cipher](https://github.com/RustCrypto/traits) | 0.4.4 | MIT OR Apache-2.0 |
| [clang-sys](https://github.com/KyleMayes/clang-sys) | 1.9.1 | Apache-2.0 |
| [clipboard-win](https://github.com/DoumanAsh/clipboard-win) | 5.4.1 | BSL-1.0 |
| [cmake](https://github.com/rust-lang/cmake-rs) | 0.1.58 | MIT OR Apache-2.0 |
| [color_quant](https://github.com/image-rs/color_quant.git) | 1.1.0 | MIT |
| [cookie](https://github.com/SergioBenitez/cookie-rs) | 0.18.2 | MIT OR Apache-2.0 |
| [core-foundation](https://github.com/servo/core-foundation-rs) | 0.10.1 | MIT OR Apache-2.0 |
| [core-foundation-sys](https://github.com/servo/core-foundation-rs) | 0.8.7 | MIT OR Apache-2.0 |
| [core-graphics](https://github.com/servo/core-foundation-rs) | 0.25.0 | MIT OR Apache-2.0 |
| [core-graphics-types](https://github.com/servo/core-foundation-rs) | 0.2.0 | MIT OR Apache-2.0 |
| [core_detect](https://github.com/thomcc/core_detect) | 1.0.0 | MIT/Apache-2.0 |
| [core_maths](https://github.com/robertbastian/core_maths) | 0.1.1 | MIT |
| [cpufeatures](https://github.com/RustCrypto/utils) | 0.2.17 | MIT OR Apache-2.0 |
| [cpufeatures](https://github.com/RustCrypto/utils) | 0.3.1 | MIT OR Apache-2.0 |
| [crc32fast](https://github.com/srijs/rust-crc32fast) | 1.5.2 | MIT OR Apache-2.0 |
| [crossbeam-channel](https://github.com/crossbeam-rs/crossbeam) | 0.5.17 | MIT OR Apache-2.0 |
| [crossbeam-utils](https://github.com/crossbeam-rs/crossbeam) | 0.8.23 | MIT OR Apache-2.0 |
| [crypto-common](https://github.com/RustCrypto/traits) | 0.1.7 | MIT OR Apache-2.0 |
| [cssparser](https://github.com/servo/rust-cssparser) | 0.37.0 | MPL-2.0 |
| [cssparser-macros](https://github.com/servo/rust-cssparser) | 0.7.1 | MPL-2.0 |
| [ctor](https://github.com/mmastrac/linktime) | 1.0.13 | Apache-2.0 OR MIT |
| [darling](https://github.com/TedDriggs/darling) | 0.24.1 | MIT |
| [darling_core](https://github.com/TedDriggs/darling) | 0.24.1 | MIT |
| [darling_macro](https://github.com/TedDriggs/darling) | 0.24.1 | MIT |
| [defmt](https://github.com/knurling-rs/defmt) | 1.1.1 | MIT OR Apache-2.0 |
| [defmt-macros](https://github.com/knurling-rs/defmt) | 1.1.1 | MIT OR Apache-2.0 |
| [defmt-parser](https://github.com/knurling-rs/defmt) | 1.0.0 | MIT OR Apache-2.0 |
| [deranged](https://github.com/jhpratt/deranged) | 0.5.8 | MIT OR Apache-2.0 |
| [derive_more](https://github.com/JelteF/derive_more) | 2.1.1 | MIT |
| [derive_more-impl](https://github.com/JelteF/derive_more) | 2.1.1 | MIT |
| [digest](https://github.com/RustCrypto/traits) | 0.10.7 | MIT OR Apache-2.0 |
| [dirs](https://github.com/soc/dirs-rs) | 5.0.1 | MIT OR Apache-2.0 |
| [dirs](https://codeberg.org/dirs/dirs-rs) | 7.0.0 | MIT OR Apache-2.0 |
| [dirs-sys](https://github.com/dirs-dev/dirs-sys-rs) | 0.4.1 | MIT OR Apache-2.0 |
| [dirs-sys](https://github.com/dirs-dev/dirs-sys-rs) | 0.5.0 | MIT OR Apache-2.0 |
| [dispatch2](https://github.com/madsmtm/objc2) | 0.3.1 | Zlib OR Apache-2.0 OR MIT |
| [displaydoc](https://github.com/yaahc/displaydoc) | 0.2.7 | MIT OR Apache-2.0 |
| [docx-rs](https://github.com/bokuweb/docx-rs) | 0.4.22 | MIT |
| [dom_query](https://github.com/niklak/dom_query) | 0.28.0 | MIT |
| [dpi](https://github.com/rust-windowing/winit) | 0.1.2 | Apache-2.0 AND MIT |
| [dtoa](https://github.com/dtolnay/dtoa) | 1.0.11 | MIT OR Apache-2.0 |
| [dtoa-short](https://github.com/upsuper/dtoa-short) | 0.3.5 | MPL-2.0 |
| [dunce](https://gitlab.com/kornelski/dunce) | 1.0.5 | CC0-1.0 OR MIT-0 OR Apache-2.0 |
| [dyn-clone](https://github.com/dtolnay/dyn-clone) | 1.0.20 | MIT OR Apache-2.0 |
| [ecb](https://github.com/magic-akari/ecb) | 0.1.2 | MIT |
| [either](https://github.com/rayon-rs/either) | 1.18.0 | MIT OR Apache-2.0 |
| [embed-resource](https://github.com/nabijaczleweli/rust-embed-resource) | 3.0.12 | MIT |
| [embed_plist](https://github.com/nvzqz/embed-plist-rs) | 1.2.2 | MIT OR Apache-2.0 |
| [encoding_rs](https://github.com/hsivonen/encoding_rs) | 0.8.42 | (Apache-2.0 OR MIT) AND BSD-3-Clause |
| [equivalent](https://github.com/indexmap-rs/equivalent) | 1.0.2 | Apache-2.0 OR MIT |
| [erased-serde](https://github.com/dtolnay/erased-serde) | 0.4.10 | MIT OR Apache-2.0 |
| [errno](https://github.com/lambda-fairy/rust-errno) | 0.3.14 | MIT OR Apache-2.0 |
| [error-code](https://github.com/DoumanAsh/error-code) | 3.4.0 | BSL-1.0 |
| [euclid](https://github.com/servo/euclid) | 0.20.14 | MIT / Apache-2.0 |
| [euclid](https://github.com/servo/euclid) | 0.22.14 | MIT OR Apache-2.0 |
| [extended](https://github.com/depp/extended-rs) | 0.1.0 | MIT |
| [eyre](https://github.com/eyre-rs/eyre) | 0.6.14 | MIT OR Apache-2.0 |
| [fastrand](https://github.com/smol-rs/fastrand) | 2.5.0 | Apache-2.0 OR MIT |
| [fax](https://github.com/pdf-rs/fax) | 0.2.7 | MIT |
| [fdeflate](https://github.com/image-rs/fdeflate) | 0.3.7 | MIT OR Apache-2.0 |
| [filetime](https://github.com/alexcrichton/filetime) | 0.2.29 | MIT/Apache-2.0 |
| [find-msvc-tools](https://github.com/rust-lang/cc-rs) | 0.1.14 | MIT OR Apache-2.0 |
| [flate2](https://github.com/rust-lang/flate2-rs) | 1.1.10 | MIT OR Apache-2.0 |
| [float-cmp](https://github.com/mikedilger/float-cmp) | 0.9.0 | MIT |
| [fnv](https://github.com/servo/rust-fnv) | 1.0.7 | Apache-2.0 / MIT |
| [foldhash](https://github.com/orlp/foldhash) | 0.2.0 | Zlib |
| [font-types](https://github.com/googlefonts/fontations) | 0.11.3 | MIT OR Apache-2.0 |
| [foreign-types](https://github.com/sfackler/foreign-types) | 0.5.0 | MIT/Apache-2.0 |
| [foreign-types-macros](https://github.com/sfackler/foreign-types) | 0.2.4 | MIT/Apache-2.0 |
| [foreign-types-shared](https://github.com/sfackler/foreign-types) | 0.3.1 | MIT/Apache-2.0 |
| [form_urlencoded](https://github.com/servo/rust-url) | 1.2.2 | MIT OR Apache-2.0 |
| [fs_extra](https://github.com/webdesus/fs_extra) | 1.3.0 | MIT |
| [generic-array](https://github.com/fizyk20/generic-array.git) | 0.14.7 | MIT |
| [getrandom](https://github.com/rust-random/getrandom) | 0.2.17 | MIT OR Apache-2.0 |
| [getrandom](https://github.com/rust-random/getrandom) | 0.3.4 | MIT OR Apache-2.0 |
| [getrandom](https://github.com/rust-random/getrandom) | 0.4.3 | MIT OR Apache-2.0 |
| [gif](https://github.com/image-rs/image-gif) | 0.14.2 | MIT OR Apache-2.0 |
| [glob](https://github.com/rust-lang/glob) | 0.3.4 | MIT OR Apache-2.0 |
| [half](https://github.com/VoidStarKat/half-rs) | 2.7.1 | MIT OR Apache-2.0 |
| [hashbrown](https://github.com/rust-lang/hashbrown) | 0.12.3 | MIT OR Apache-2.0 |
| [hashbrown](https://github.com/rust-lang/hashbrown) | 0.17.1 | MIT OR Apache-2.0 |
| [heck](https://github.com/withoutboats/heck) | 0.5.0 | MIT OR Apache-2.0 |
| [hex](https://github.com/KokaKiwi/rust-hex) | 0.4.3 | MIT OR Apache-2.0 |
| [home](https://github.com/rust-lang/cargo) | 0.5.12 | MIT OR Apache-2.0 |
| [hound](https://github.com/ruuda/hound) | 3.5.1 | Apache-2.0 |
| [html5ever](https://github.com/servo/html5ever) | 0.39.0 | MIT OR Apache-2.0 |
| [http](https://github.com/hyperium/http) | 1.5.0 | MIT OR Apache-2.0 |
| [http-range](https://github.com/bancek/rust-http-range.git) | 0.1.5 | MIT |
| [httparse](https://github.com/seanmonstar/httparse) | 1.10.1 | MIT OR Apache-2.0 |
| [iana-time-zone](https://github.com/strawlab/iana-time-zone) | 0.1.65 | MIT OR Apache-2.0 |
| [ico](https://github.com/mdsteele/rust-ico) | 0.5.0 | MIT |
| [icu_collections](https://github.com/unicode-org/icu4x) | 2.3.0 | Unicode-3.0 |
| [icu_locale_core](https://github.com/unicode-org/icu4x) | 2.3.0 | Unicode-3.0 |
| [icu_normalizer](https://github.com/unicode-org/icu4x) | 2.3.0 | Unicode-3.0 |
| [icu_normalizer_data](https://github.com/unicode-org/icu4x) | 2.3.0 | Unicode-3.0 |
| [icu_properties](https://github.com/unicode-org/icu4x) | 2.3.0 | Unicode-3.0 |
| [icu_properties_data](https://github.com/unicode-org/icu4x) | 2.3.0 | Unicode-3.0 |
| [icu_provider](https://github.com/unicode-org/icu4x) | 2.3.1 | Unicode-3.0 |
| [ident_case](https://github.com/TedDriggs/ident_case) | 1.0.1 | MIT/Apache-2.0 |
| [idna](https://github.com/servo/rust-url/) | 1.1.0 | MIT OR Apache-2.0 |
| [idna_adapter](https://github.com/hsivonen/idna_adapter) | 1.2.2 | Apache-2.0 OR MIT |
| [image](https://github.com/image-rs/image) | 0.25.10 | MIT OR Apache-2.0 |
| [image-webp](https://github.com/image-rs/image-webp) | 0.2.4 | MIT OR Apache-2.0 |
| [imagesize](https://github.com/Roughsketch/imagesize) | 0.14.0 | MIT |
| [indenter](https://github.com/yaahc/indenter) | 0.3.4 | MIT OR Apache-2.0 |
| [indexmap](https://github.com/bluss/indexmap) | 1.9.3 | Apache-2.0 OR MIT |
| [indexmap](https://github.com/indexmap-rs/indexmap) | 2.14.2 | Apache-2.0 OR MIT |
| [infer](https://github.com/bojand/infer) | 0.22.0 | MIT |
| [inout](https://github.com/RustCrypto/utils) | 0.1.4 | MIT OR Apache-2.0 |
| [itertools](https://github.com/rust-itertools/itertools) | 0.12.1 | MIT OR Apache-2.0 |
| [itertools](https://github.com/rust-itertools/itertools) | 0.13.0 | MIT OR Apache-2.0 |
| [itoa](https://github.com/dtolnay/itoa) | 1.0.18 | MIT OR Apache-2.0 |
| [jiff](https://github.com/BurntSushi/jiff) | 0.2.37 | Unlicense OR MIT |
| [jiff-core](https://github.com/BurntSushi/jiff) | 0.1.1 | Unlicense OR MIT |
| [jiff-tzdb](https://github.com/BurntSushi/jiff) | 0.1.8 | Unlicense OR MIT |
| [jiff-tzdb-platform](https://github.com/BurntSushi/jiff) | 0.1.3 | Unlicense OR MIT |
| [json-patch](https://github.com/idubrov/json-patch) | 4.2.0 | MIT/Apache-2.0 |
| [jsonptr](https://github.com/chanced/jsonptr) | 0.7.1 | MIT OR Apache-2.0 |
| [keyboard-types](https://github.com/rust-windowing/keyboard-types) | 0.8.3 | MIT OR Apache-2.0 |
| [krilla](https://github.com/LaurenzV/krilla) | 0.8.2 | MIT OR Apache-2.0 |
| [kurbo](https://github.com/linebender/kurbo) | 0.13.1 | Apache-2.0 OR MIT |
| [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs) | 1.5.1 | MIT OR Apache-2.0 |
| [lazycell](https://github.com/indiv0/lazycell) | 1.3.0 | MIT/Apache-2.0 |
| [libc](https://github.com/rust-lang/libc) | 0.2.190 | MIT OR Apache-2.0 |
| [libloading](https://github.com/nagisa/rust_libloading/) | 0.8.9 | ISC |
| [libm](https://github.com/rust-lang/compiler-builtins) | 0.2.16 | MIT |
| [litemap](https://github.com/unicode-org/icu4x) | 0.8.3 | Unicode-3.0 |
| [lock_api](https://github.com/Amanieu/parking_lot) | 0.4.14 | MIT OR Apache-2.0 |
| [log](https://github.com/rust-lang/log) | 0.4.34 | MIT OR Apache-2.0 |
| [lopdf](https://github.com/J-F-Liu/lopdf.git) | 0.42.0 | MIT |
| [markup5ever](https://github.com/servo/html5ever) | 0.39.0 | MIT OR Apache-2.0 |
| [md-5](https://github.com/RustCrypto/hashes) | 0.10.6 | MIT OR Apache-2.0 |
| [memchr](https://github.com/BurntSushi/memchr) | 2.8.3 | Unlicense OR MIT |
| [memmap2](https://github.com/RazrFalcon/memmap2-rs) | 0.9.11 | MIT OR Apache-2.0 |
| [mime](https://github.com/hyperium/mime) | 0.3.17 | MIT OR Apache-2.0 |
| [minimal-lexical](https://github.com/Alexhuszagh/minimal-lexical) | 0.2.1 | MIT/Apache-2.0 |
| [miniz_oxide](https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide) | 0.8.9 | MIT OR Zlib OR Apache-2.0 |
| [miniz_oxide](https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide) | 0.9.1 | MIT OR Zlib OR Apache-2.0 |
| [mio](https://github.com/tokio-rs/mio) | 1.2.4 | MIT |
| [moxcms](https://github.com/awxkee/moxcms.git) | 0.8.1 | BSD-3-Clause OR Apache-2.0 |
| [muda](https://github.com/tauri-apps/muda) | 0.20.0 | Apache-2.0 OR MIT |
| [multiversion_no_op](https://github.com/hsivonen/multiversion_no_op) | 1.0.0 | Apache-2.0 OR MIT |
| [new_debug_unreachable](https://github.com/mbrubeck/rust-debug-unreachable) | 1.0.6 | MIT |
| [nom](https://github.com/Geal/nom) | 7.1.3 | MIT |
| [nom](https://github.com/rust-bakery/nom) | 8.0.0 | MIT |
| [num-complex](https://github.com/rust-num/num-complex) | 0.4.6 | MIT OR Apache-2.0 |
| [num-conv](https://github.com/jhpratt/num-conv) | 0.2.2 | MIT OR Apache-2.0 |
| [num-integer](https://github.com/rust-num/num-integer) | 0.1.47 | MIT OR Apache-2.0 |
| [num-traits](https://github.com/rust-num/num-traits) | 0.2.19 | MIT OR Apache-2.0 |
| [objc2](https://github.com/madsmtm/objc2) | 0.6.5 | MIT |
| [objc2-app-kit](https://github.com/madsmtm/objc2) | 0.3.2 | Zlib OR Apache-2.0 OR MIT |
| [objc2-core-foundation](https://github.com/madsmtm/objc2) | 0.3.2 | Zlib OR Apache-2.0 OR MIT |
| [objc2-core-graphics](https://github.com/madsmtm/objc2) | 0.3.2 | Zlib OR Apache-2.0 OR MIT |
| [objc2-encode](https://github.com/madsmtm/objc2) | 4.1.0 | MIT |
| [objc2-exception-helper](https://github.com/madsmtm/objc2) | 0.1.1 | Zlib OR Apache-2.0 OR MIT |
| [objc2-foundation](https://github.com/madsmtm/objc2) | 0.3.2 | MIT |
| [objc2-io-surface](https://github.com/madsmtm/objc2) | 0.3.2 | Zlib OR Apache-2.0 OR MIT |
| [objc2-quartz-core](https://github.com/madsmtm/objc2) | 0.3.2 | Zlib OR Apache-2.0 OR MIT |
| [objc2-web-kit](https://github.com/madsmtm/objc2) | 0.3.2 | Zlib OR Apache-2.0 OR MIT |
| [once_cell](https://github.com/matklad/once_cell) | 1.21.4 | MIT OR Apache-2.0 |
| [open](https://github.com/Byron/open-rs) | 5.4.4 | MIT |
| [option-ext](https://github.com/soc/option-ext.git) | 0.2.0 | MPL-2.0 |
| [opusic-sys](https://github.com/DoumanAsh/opusic-sys) | 0.7.5 | BSD-3-Clause |
| [parking_lot](https://github.com/Amanieu/parking_lot) | 0.12.5 | MIT OR Apache-2.0 |
| [parking_lot_core](https://github.com/Amanieu/parking_lot) | 0.9.12 | MIT OR Apache-2.0 |
| [pdf-extract](https://github.com/jrmuizel/pdf-extract) | 0.12.1 | MIT |
| [pdf-writer](https://github.com/typst/pdf-writer) | 0.15.0 | MIT OR Apache-2.0 |
| [percent-encoding](https://github.com/servo/rust-url/) | 2.3.2 | MIT OR Apache-2.0 |
| [phf](https://github.com/rust-phf/rust-phf) | 0.13.1 | MIT |
| [phf_codegen](https://github.com/rust-phf/rust-phf) | 0.13.1 | MIT |
| [phf_generator](https://github.com/rust-phf/rust-phf) | 0.13.1 | MIT |
| [phf_macros](https://github.com/rust-phf/rust-phf) | 0.13.1 | MIT |
| [phf_shared](https://github.com/rust-phf/rust-phf) | 0.13.1 | MIT |
| [pin-project-lite](https://github.com/taiki-e/pin-project-lite) | 0.2.17 | Apache-2.0 OR MIT |
| [pkg-config](https://github.com/rust-lang/pkg-config-rs) | 0.3.34 | MIT OR Apache-2.0 |
| [plist](https://github.com/ebarnard/rust-plist/) | 1.10.1 | MIT |
| [png](https://github.com/image-rs/image-png) | 0.17.16 | MIT OR Apache-2.0 |
| [png](https://github.com/image-rs/image-png) | 0.18.1 | MIT OR Apache-2.0 |
| [polycool](https://github.com/linebender/kurbo) | 0.4.0 | MIT OR Apache-2.0 |
| [pom](https://github.com/J-F-Liu/pom.git) | 1.1.0 | MIT |
| [postscript](https://github.com/bodoni/postscript) | 0.14.1 | Apache-2.0/MIT |
| [potential_utf](https://github.com/unicode-org/icu4x) | 0.1.6 | Unicode-3.0 |
| [powerfmt](https://github.com/jhpratt/powerfmt) | 0.2.1 | MIT OR Apache-2.0 |
| [precomputed-hash](https://github.com/emilio/precomputed-hash) | 0.1.1 | MIT |
| [prettyplease](https://github.com/dtolnay/prettyplease) | 0.2.37 | MIT OR Apache-2.0 |
| [primal-check](https://github.com/huonw/primal) | 0.3.4 | MIT OR Apache-2.0 |
| [proc-macro2](https://github.com/dtolnay/proc-macro2) | 1.0.107 | MIT OR Apache-2.0 |
| [pxfm](https://github.com/awxkee/pxfm) | 0.1.30 | BSD-3-Clause OR Apache-2.0 |
| [quick-error](http://github.com/tailhook/quick-error) | 2.0.1 | MIT/Apache-2.0 |
| [quick-xml](https://github.com/tafia/quick-xml) | 0.41.0 | MIT |
| [quick-xml](https://github.com/tafia/quick-xml) | 0.42.0 | MIT |
| [quote](https://github.com/dtolnay/quote) | 1.0.47 | MIT OR Apache-2.0 |
| [rand](https://github.com/rust-random/rand) | 0.10.3 | MIT OR Apache-2.0 |
| [rand_core](https://github.com/rust-random/rand_core) | 0.10.1 | MIT OR Apache-2.0 |
| [rangemap](https://github.com/jeffparsons/rangemap) | 1.8.0 | MIT/Apache-2.0 |
| [raw-window-handle](https://github.com/rust-windowing/raw-window-handle) | 0.6.2 | MIT OR Apache-2.0 OR Zlib |
| [read-fonts](https://github.com/googlefonts/fontations) | 0.39.2 | MIT OR Apache-2.0 |
| [realfft](https://github.com/HEnquist/realfft) | 3.5.0 | MIT |
| [ref-cast](https://github.com/dtolnay/ref-cast) | 1.0.27 | MIT OR Apache-2.0 |
| [ref-cast-impl](https://github.com/dtolnay/ref-cast) | 1.0.27 | MIT OR Apache-2.0 |
| [regex](https://github.com/rust-lang/regex) | 1.13.1 | MIT OR Apache-2.0 |
| [regex-automata](https://github.com/rust-lang/regex) | 0.4.18 | MIT OR Apache-2.0 |
| [regex-lite](https://github.com/rust-lang/regex) | 0.1.9 | MIT OR Apache-2.0 |
| [regex-syntax](https://github.com/rust-lang/regex) | 0.8.11 | MIT OR Apache-2.0 |
| [rfd](https://github.com/PolyMeilex/rfd) | 0.16.0 | MIT |
| [ring](https://github.com/briansmith/ring) | 0.17.14 | Apache-2.0 AND ISC |
| [rubato](https://github.com/HEnquist/rubato) | 0.16.2 | MIT |
| [rustc-hash](https://github.com/rust-lang-nursery/rustc-hash) | 1.1.0 | Apache-2.0/MIT |
| [rustc-hash](https://github.com/rust-lang/rustc-hash) | 2.1.3 | Apache-2.0 OR MIT |
| [rustc_version](https://github.com/djc/rustc-version-rs) | 0.4.1 | MIT OR Apache-2.0 |
| [rustfft](https://github.com/ejmahler/RustFFT) | 6.4.1 | MIT OR Apache-2.0 |
| [rustix](https://github.com/bytecodealliance/rustix) | 0.38.44 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT |
| [rustix](https://github.com/bytecodealliance/rustix) | 1.1.5 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT |
| [rustls](https://github.com/rustls/rustls) | 0.23.45 | Apache-2.0 OR ISC OR MIT |
| [rustls-pki-types](https://github.com/rustls/pki-types) | 1.15.1 | MIT OR Apache-2.0 |
| [rustls-webpki](https://github.com/rustls/webpki) | 0.103.15 | ISC |
| [rustversion](https://github.com/dtolnay/rustversion) | 1.0.23 | MIT OR Apache-2.0 |
| [rustybuzz](https://github.com/harfbuzz/rustybuzz) | 0.20.1 | MIT |
| [ryu](https://github.com/dtolnay/ryu) | 1.0.23 | Apache-2.0 OR BSL-1.0 |
| [same-file](https://github.com/BurntSushi/same-file) | 1.0.6 | Unlicense/MIT |
| [schemars](https://github.com/GREsau/schemars) | 0.8.22 | MIT |
| [schemars](https://github.com/GREsau/schemars) | 0.9.0 | MIT |
| [schemars](https://github.com/GREsau/schemars) | 1.2.2 | MIT |
| [schemars_derive](https://github.com/GREsau/schemars) | 0.8.22 | MIT |
| [scopeguard](https://github.com/bluss/scopeguard) | 1.2.0 | MIT OR Apache-2.0 |
| [selectors](https://github.com/servo/stylo) | 0.38.0 | MPL-2.0 |
| [semver](https://github.com/dtolnay/semver) | 1.0.28 | MIT OR Apache-2.0 |
| [serde](https://github.com/serde-rs/serde) | 1.0.229 | MIT OR Apache-2.0 |
| [serde-untagged](https://github.com/dtolnay/serde-untagged) | 0.1.9 | MIT OR Apache-2.0 |
| [serde_core](https://github.com/serde-rs/serde) | 1.0.229 | MIT OR Apache-2.0 |
| [serde_derive](https://github.com/serde-rs/serde) | 1.0.229 | MIT OR Apache-2.0 |
| [serde_derive_internals](https://github.com/serde-rs/serde) | 0.29.1 | MIT OR Apache-2.0 |
| [serde_json](https://github.com/serde-rs/json) | 1.0.151 | MIT OR Apache-2.0 |
| [serde_repr](https://github.com/dtolnay/serde-repr) | 0.1.21 | MIT OR Apache-2.0 |
| [serde_spanned](https://github.com/toml-rs/toml) | 1.1.1 | MIT OR Apache-2.0 |
| [serde_with](https://github.com/jonasbb/serde_with/) | 3.24.0 | MIT OR Apache-2.0 |
| [serde_with_macros](https://github.com/jonasbb/serde_with/) | 3.24.0 | MIT OR Apache-2.0 |
| [serialize-to-javascript](https://github.com/chippers/serialize-to-javascript) | 0.1.2 | MIT OR Apache-2.0 |
| [serialize-to-javascript-impl](https://github.com/chippers/serialize-to-javascript) | 0.1.2 | MIT OR Apache-2.0 |
| [servo_arc](https://github.com/servo/stylo) | 0.4.3 | MIT OR Apache-2.0 |
| [sha2](https://github.com/RustCrypto/hashes) | 0.10.9 | MIT OR Apache-2.0 |
| [sherpa-rs](https://github.com/thewh1teagle/sherpa-rs) | 0.6.8 | MIT |
| [sherpa-rs-sys](https://github.com/thewh1teagle/sherpa-rs) | 0.6.8 | MIT |
| [shlex](https://github.com/comex/rust-shlex) | 1.3.0 | MIT OR Apache-2.0 |
| [shlex](https://github.com/comex/rust-shlex) | 2.0.1 | MIT OR Apache-2.0 |
| [simd-adler32](https://github.com/mcountryman/simd-adler32) | 0.3.10 | MIT |
| [simdutf8](https://github.com/rusticstuff/simdutf8) | 0.1.5 | MIT OR Apache-2.0 |
| [siphasher](https://github.com/jedisct1/rust-siphash) | 1.0.4 | MIT OR Apache-2.0 |
| [skrifa](https://github.com/googlefonts/fontations) | 0.42.1 | MIT OR Apache-2.0 |
| [smallvec](https://github.com/servo/rust-smallvec) | 1.16.2 | MIT OR Apache-2.0 |
| [socket2](https://github.com/rust-lang/socket2) | 0.6.5 | MIT OR Apache-2.0 |
| [socks](https://github.com/sfackler/rust-socks) | 0.3.4 | MIT/Apache-2.0 |
| [softbuffer](https://github.com/rust-windowing/softbuffer) | 0.4.8 | MIT OR Apache-2.0 |
| [stable_deref_trait](https://github.com/storyyeller/stable_deref_trait) | 1.2.1 | MIT OR Apache-2.0 |
| [strength_reduce](http://github.com/ejmahler/strength_reduce) | 0.2.4 | MIT OR Apache-2.0 |
| [strict-num](https://github.com/RazrFalcon/strict-num) | 0.1.1 | MIT |
| [string_cache](https://github.com/servo/string-cache) | 0.9.0 | MIT OR Apache-2.0 |
| [string_cache_codegen](https://github.com/servo/string-cache) | 0.6.1 | MIT OR Apache-2.0 |
| [stringprep](https://github.com/sfackler/rust-stringprep) | 0.1.5 | MIT/Apache-2.0 |
| [strsim](https://github.com/rapidfuzz/strsim-rs) | 0.11.1 | MIT |
| [subsetter](https://github.com/typst/subsetter) | 0.2.6 | MIT OR Apache-2.0 |
| [subtle](https://github.com/dalek-cryptography/subtle) | 2.6.1 | BSD-3-Clause |
| [swift-rs](https://github.com/Brendonovich/swift-rs) | 1.0.8 | MIT OR Apache-2.0 |
| [symphonia](https://github.com/pdeljanov/Symphonia) | 0.6.1 | MPL-2.0 |
| [symphonia-adapter-libopus](https://github.com/aschey/symphonia-adapters) | 0.3.0 | MIT OR Apache-2.0 |
| [symphonia-bundle-flac](https://github.com/pdeljanov/Symphonia) | 0.6.1 | MPL-2.0 |
| [symphonia-bundle-mp3](https://github.com/pdeljanov/Symphonia) | 0.6.1 | MPL-2.0 |
| [symphonia-codec-aac](https://github.com/pdeljanov/Symphonia) | 0.6.1 | MPL-2.0 |
| [symphonia-codec-adpcm](https://github.com/pdeljanov/Symphonia) | 0.6.1 | MPL-2.0 |
| [symphonia-codec-alac](https://github.com/pdeljanov/Symphonia) | 0.6.1 | MPL-2.0 |
| [symphonia-codec-pcm](https://github.com/pdeljanov/Symphonia) | 0.6.1 | MPL-2.0 |
| [symphonia-codec-vorbis](https://github.com/pdeljanov/Symphonia) | 0.6.1 | MPL-2.0 |
| [symphonia-common](https://github.com/pdeljanov/Symphonia) | 0.6.1 | MPL-2.0 |
| [symphonia-core](https://github.com/pdeljanov/Symphonia) | 0.6.1 | MPL-2.0 |
| [symphonia-format-caf](https://github.com/pdeljanov/Symphonia) | 0.6.1 | MPL-2.0 |
| [symphonia-format-isomp4](https://github.com/pdeljanov/Symphonia) | 0.6.1 | MPL-2.0 |
| [symphonia-format-mkv](https://github.com/pdeljanov/Symphonia) | 0.6.1 | MPL-2.0 |
| [symphonia-format-ogg](https://github.com/pdeljanov/Symphonia) | 0.6.1 | MPL-2.0 |
| [symphonia-format-riff](https://github.com/pdeljanov/Symphonia) | 0.6.1 | MPL-2.0 |
| [symphonia-metadata](https://github.com/pdeljanov/Symphonia) | 0.6.1 | MPL-2.0 |
| [syn](https://github.com/dtolnay/syn) | 2.0.119 | MIT OR Apache-2.0 |
| [syn](https://github.com/dtolnay/syn) | 3.0.6 | MIT OR Apache-2.0 |
| [synstructure](https://github.com/mystor/synstructure) | 0.14.0 | MIT |
| [tao](https://github.com/tauri-apps/tao) | 0.37.1 | Apache-2.0 |
| [tar](https://github.com/composefs/tar-rs) | 0.4.46 | MIT OR Apache-2.0 |
| [tauri](https://github.com/tauri-apps/tauri) | 2.12.1 | Apache-2.0 OR MIT |
| [tauri-build](https://github.com/tauri-apps/tauri) | 2.7.1 | Apache-2.0 OR MIT |
| [tauri-codegen](https://github.com/tauri-apps/tauri) | 2.7.1 | Apache-2.0 OR MIT |
| [tauri-macros](https://github.com/tauri-apps/tauri) | 2.7.1 | Apache-2.0 OR MIT |
| [tauri-plugin](https://github.com/tauri-apps/tauri) | 2.7.1 | Apache-2.0 OR MIT |
| [tauri-plugin-clipboard-manager](https://github.com/tauri-apps/plugins-workspace) | 2.4.1 | Apache-2.0 OR MIT |
| [tauri-plugin-dialog](https://github.com/tauri-apps/plugins-workspace) | 2.8.1 | Apache-2.0 OR MIT |
| [tauri-plugin-fs](https://github.com/tauri-apps/plugins-workspace) | 2.6.0 | Apache-2.0 OR MIT |
| [tauri-plugin-opener](https://github.com/tauri-apps/plugins-workspace) | 2.7.0 | Apache-2.0 OR MIT |
| [tauri-runtime](https://github.com/tauri-apps/tauri) | 2.12.1 | Apache-2.0 OR MIT |
| [tauri-runtime-wry](https://github.com/tauri-apps/tauri) | 2.12.1 | Apache-2.0 OR MIT |
| [tauri-utils](https://github.com/tauri-apps/tauri) | 2.10.1 | Apache-2.0 OR MIT |
| [tauri-winres](https://github.com/tauri-apps/winres) | 0.3.6 | MIT |
| [tempfile](https://github.com/Stebalien/tempfile) | 3.27.0 | MIT OR Apache-2.0 |
| [tendril](https://github.com/servo/html5ever) | 0.5.1 | MIT OR Apache-2.0 |
| [thiserror](https://github.com/dtolnay/thiserror) | 2.0.21 | MIT OR Apache-2.0 |
| [thiserror-impl](https://github.com/dtolnay/thiserror) | 2.0.21 | MIT OR Apache-2.0 |
| [tiff](https://github.com/image-rs/image-tiff) | 0.11.3 | MIT |
| [time](https://github.com/time-rs/time) | 0.3.55 | MIT OR Apache-2.0 |
| [time-core](https://github.com/time-rs/time) | 0.1.9 | MIT OR Apache-2.0 |
| [time-macros](https://github.com/time-rs/time) | 0.2.32 | MIT OR Apache-2.0 |
| [tiny-skia-path](https://github.com/linebender/tiny-skia/tree/master/path) | 0.12.0 | BSD-3-Clause |
| [tinystr](https://github.com/unicode-org/icu4x) | 0.8.4 | Unicode-3.0 |
| [tinyvec](https://github.com/Lokathor/tinyvec) | 1.13.3 | Zlib OR Apache-2.0 OR MIT |
| [tokio](https://github.com/tokio-rs/tokio) | 1.53.2 | MIT |
| [toml](https://github.com/toml-rs/toml) | 1.1.6+spec-1.1.0 | MIT OR Apache-2.0 |
| [toml_datetime](https://github.com/toml-rs/toml) | 1.1.1+spec-1.1.0 | MIT OR Apache-2.0 |
| [toml_parser](https://github.com/toml-rs/toml) | 1.1.3+spec-1.1.0 | MIT OR Apache-2.0 |
| [toml_writer](https://github.com/toml-rs/toml) | 1.1.2+spec-1.1.0 | MIT OR Apache-2.0 |
| [tracing](https://github.com/tokio-rs/tracing) | 0.1.44 | MIT |
| [tracing-attributes](https://github.com/tokio-rs/tracing) | 0.1.31 | MIT |
| [tracing-core](https://github.com/tokio-rs/tracing) | 0.1.36 | MIT |
| [transpose](https://github.com/ejmahler/transpose) | 0.2.3 | MIT OR Apache-2.0 |
| [tray-icon](https://github.com/tauri-apps/tray-icon) | 0.25.1 | MIT OR Apache-2.0 |
| [ttf-parser](https://github.com/harfbuzz/ttf-parser) | 0.25.1 | MIT OR Apache-2.0 |
| [type1-encoding-parser](https://github.com/jrmuizel/type1-encoding-parser) | 0.1.1 | MIT |
| [typed-path](https://github.com/chipsenkbeil/typed-path) | 0.12.3 | MIT OR Apache-2.0 |
| [typeid](https://github.com/dtolnay/typeid) | 1.0.3 | MIT OR Apache-2.0 |
| [typenum](https://github.com/paholg/typenum) | 1.20.1 | MIT OR Apache-2.0 |
| [unicode-bidi](https://github.com/servo/unicode-bidi) | 0.3.18 | MIT OR Apache-2.0 |
| [unicode-bidi-mirroring](https://github.com/RazrFalcon/unicode-bidi-mirroring) | 0.4.0 | MIT/Apache-2.0 |
| [unicode-ccc](https://github.com/RazrFalcon/unicode-ccc) | 0.4.0 | MIT/Apache-2.0 |
| [unicode-ident](https://github.com/dtolnay/unicode-ident) | 1.0.26 | (MIT OR Apache-2.0) AND Unicode-3.0 |
| [unicode-normalization](https://github.com/unicode-rs/unicode-normalization) | 0.1.25 | MIT OR Apache-2.0 |
| [unicode-properties](https://github.com/unicode-rs/unicode-properties) | 0.1.4 | MIT/Apache-2.0 |
| [unicode-script](https://github.com/unicode-rs/unicode-script) | 0.5.8 | MIT OR Apache-2.0 |
| [unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation) | 1.13.3 | MIT OR Apache-2.0 |
| [untrusted](https://github.com/briansmith/untrusted) | 0.9.0 | ISC |
| [ureq](https://github.com/algesten/ureq) | 2.12.1 | MIT OR Apache-2.0 |
| [ureq](https://github.com/algesten/ureq) | 3.4.2 | MIT OR Apache-2.0 |
| [ureq-proto](https://github.com/algesten/ureq-proto) | 0.6.4 | MIT OR Apache-2.0 |
| [url](https://github.com/servo/rust-url) | 2.5.8 | MIT OR Apache-2.0 |
| [urlpattern](https://github.com/denoland/rust-urlpattern) | 0.6.0 | MIT |
| [utf8-zero](https://github.com/algesten/utf8-zero) | 0.8.1 | MIT OR Apache-2.0 |
| [utf8_iter](https://github.com/hsivonen/utf8_iter) | 1.0.4 | Apache-2.0 OR MIT |
| [uuid](https://github.com/uuid-rs/uuid) | 1.27.0 | Apache-2.0 OR MIT |
| [version_check](https://github.com/SergioBenitez/version_check) | 0.9.5 | MIT/Apache-2.0 |
| [vswhom](https://github.com/nabijaczleweli/vswhom.rs) | 0.1.0 | MIT |
| [vswhom-sys](https://github.com/nabijaczleweli/vswhom-sys.rs) | 0.1.3 | MIT |
| [walkdir](https://github.com/BurntSushi/walkdir) | 2.5.0 | Unlicense/MIT |
| [web-time](https://github.com/daxpedda/web-time) | 1.1.0 | MIT OR Apache-2.0 |
| [web_atoms](https://github.com/servo/html5ever) | 0.2.6 | MIT OR Apache-2.0 |
| [webpki-roots](https://github.com/rustls/webpki-roots) | 0.26.11 | CDLA-Permissive-2.0 |
| [webpki-roots](https://github.com/rustls/webpki-roots) | 1.0.9 | CDLA-Permissive-2.0 |
| [webview2-com](https://github.com/wravery/webview2-rs) | 0.39.1 | MIT |
| [webview2-com-macros](https://github.com/wravery/webview2-rs) | 0.8.1 | MIT |
| [webview2-com-sys](https://github.com/wravery/webview2-rs) | 0.39.1 | MIT |
| [weezl](https://github.com/image-rs/weezl) | 0.1.12 | MIT OR Apache-2.0 |
| [which](https://github.com/harryfei/which-rs.git) | 4.4.2 | MIT |
| [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) | 0.16.0 | Unlicense |
| [whisper-rs-sys](https://codeberg.org/tazz4843/whisper-rs) | 0.15.0 | Unlicense |
| [winapi](https://github.com/retep998/winapi-rs) | 0.3.9 | MIT/Apache-2.0 |
| [winapi-util](https://github.com/BurntSushi/winapi-util) | 0.1.11 | Unlicense OR MIT |
| [window-vibrancy](https://github.com/tauri-apps/tauri-plugin-vibrancy) | 0.8.1 | Apache-2.0 OR MIT |
| [windows](https://github.com/microsoft/windows-rs) | 0.62.2 | MIT OR Apache-2.0 |
| [windows-collections](https://github.com/microsoft/windows-rs) | 0.3.2 | MIT OR Apache-2.0 |
| [windows-core](https://github.com/microsoft/windows-rs) | 0.62.2 | MIT OR Apache-2.0 |
| [windows-future](https://github.com/microsoft/windows-rs) | 0.3.2 | MIT OR Apache-2.0 |
| [windows-implement](https://github.com/microsoft/windows-rs) | 0.60.2 | MIT OR Apache-2.0 |
| [windows-interface](https://github.com/microsoft/windows-rs) | 0.59.3 | MIT OR Apache-2.0 |
| [windows-link](https://github.com/microsoft/windows-rs) | 0.2.1 | MIT OR Apache-2.0 |
| [windows-numerics](https://github.com/microsoft/windows-rs) | 0.3.1 | MIT OR Apache-2.0 |
| [windows-result](https://github.com/microsoft/windows-rs) | 0.4.1 | MIT OR Apache-2.0 |
| [windows-strings](https://github.com/microsoft/windows-rs) | 0.5.1 | MIT OR Apache-2.0 |
| [windows-sys](https://github.com/microsoft/windows-rs) | 0.48.0 | MIT OR Apache-2.0 |
| [windows-sys](https://github.com/microsoft/windows-rs) | 0.59.0 | MIT OR Apache-2.0 |
| [windows-sys](https://github.com/microsoft/windows-rs) | 0.60.2 | MIT OR Apache-2.0 |
| [windows-sys](https://github.com/microsoft/windows-rs) | 0.61.2 | MIT OR Apache-2.0 |
| [windows-targets](https://github.com/microsoft/windows-rs) | 0.48.5 | MIT OR Apache-2.0 |
| [windows-targets](https://github.com/microsoft/windows-rs) | 0.52.6 | MIT OR Apache-2.0 |
| [windows-targets](https://github.com/microsoft/windows-rs) | 0.53.5 | MIT OR Apache-2.0 |
| [windows-threading](https://github.com/microsoft/windows-rs) | 0.2.1 | MIT OR Apache-2.0 |
| [windows-version](https://github.com/microsoft/windows-rs) | 0.1.7 | MIT OR Apache-2.0 |
| [windows_x86_64_msvc](https://github.com/microsoft/windows-rs) | 0.48.5 | MIT OR Apache-2.0 |
| [windows_x86_64_msvc](https://github.com/microsoft/windows-rs) | 0.52.6 | MIT OR Apache-2.0 |
| [windows_x86_64_msvc](https://github.com/microsoft/windows-rs) | 0.53.1 | MIT OR Apache-2.0 |
| [winnow](https://github.com/winnow-rs/winnow) | 1.0.4 | MIT |
| [winreg](https://github.com/gentoo90/winreg-rs) | 0.56.0 | MIT |
| [write-fonts](https://github.com/googlefonts/fontations) | 0.48.1 | MIT OR Apache-2.0 |
| [writeable](https://github.com/unicode-org/icu4x) | 0.6.4 | Unicode-3.0 |
| [wry](https://github.com/tauri-apps/wry) | 0.57.0 | Apache-2.0 OR MIT |
| [xattr](https://github.com/Stebalien/xattr) | 1.6.1 | MIT OR Apache-2.0 |
| [xmp-writer](https://github.com/typst/xmp-writer) | 0.3.3 | MIT OR Apache-2.0 |
| [yoke](https://github.com/unicode-org/icu4x) | 0.8.3 | Unicode-3.0 |
| [yoke-derive](https://github.com/unicode-org/icu4x) | 0.8.4 | Unicode-3.0 |
| [zerocopy](https://github.com/google/zerocopy) | 0.8.60 | BSD-2-Clause OR Apache-2.0 OR MIT |
| [zerocopy-derive](https://github.com/google/zerocopy) | 0.8.60 | BSD-2-Clause OR Apache-2.0 OR MIT |
| [zerofrom](https://github.com/unicode-org/icu4x) | 0.1.8 | Unicode-3.0 |
| [zerofrom-derive](https://github.com/unicode-org/icu4x) | 0.1.8 | Unicode-3.0 |
| [zeroize](https://github.com/RustCrypto/utils) | 1.9.0 | Apache-2.0 OR MIT |
| [zerotrie](https://github.com/unicode-org/icu4x) | 0.2.5 | Unicode-3.0 |
| [zerovec](https://github.com/unicode-org/icu4x) | 0.11.8 | Unicode-3.0 |
| [zerovec-derive](https://github.com/unicode-org/icu4x) | 0.11.6 | Unicode-3.0 |
| [zip](https://github.com/zip-rs/zip2.git) | 2.4.2 | MIT |
| [zip](https://github.com/zip-rs/zip2) | 8.6.0 | MIT |
| [zlib-rs](https://github.com/trifectatechfoundation/zlib-rs) | 0.6.8 | Zlib |
| [zmij](https://github.com/dtolnay/zmij) | 1.0.23 | MIT |
| [zopfli](https://github.com/zopfli-rs/zopfli) | 0.8.3 | Apache-2.0 |
| [zune-core](https://github.com/etemesi254/zune-image) | 0.5.3 | MIT OR Apache-2.0 OR Zlib |
| [zune-jpeg](https://github.com/etemesi254/zune-image/tree/dev/crates/zune-jpeg) | 0.5.15 | MIT OR Apache-2.0 OR Zlib |
