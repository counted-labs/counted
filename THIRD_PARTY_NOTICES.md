# Third-party notices

Everything in this repository is covered by [LICENSE](LICENSE) (AGPL-3.0-only) except the files
below, which keep their upstream licences.

| Path | Upstream | Licence |
| --- | --- | --- |
| `vendor/tao/` | [tao](https://github.com/tauri-apps/tao) 0.34.8 — Copyright 2014-2021 The winit contributors, Copyright 2021-2023 Tauri Programme within The Commons Conservancy | Apache-2.0, [vendor/tao/LICENSE](vendor/tao/LICENSE) |
| `vendor/jni-macros/` | [jni-macros](https://github.com/jni-rs/jni-rs) 0.22.4 — Copyright the jni-rs contributors | MIT OR Apache-2.0, Apache-2.0 text in [vendor/tao/LICENSE](vendor/tao/LICENSE) |
| `packages/ocr/models/` | [PP-OCRv6_small_det](https://huggingface.co/PaddlePaddle/PP-OCRv6_small_det_onnx) and [PP-OCRv6_small_rec](https://huggingface.co/PaddlePaddle/PP-OCRv6_small_rec_onnx) — Copyright PaddlePaddle Authors | Apache-2.0, text in [vendor/tao/LICENSE](vendor/tao/LICENSE) |
| Logo and store artwork, listed in [README.md](README.md#licence) | Counted — Copyright 2022-2026 Jonathan Bosi | CC-BY-SA-4.0, [creativecommons.org/licenses/by-sa/4.0](https://creativecommons.org/licenses/by-sa/4.0/legalcode) |
| `packages/*/assets/fonts/inter-*.woff2` | [Inter](https://github.com/rsms/inter) — Copyright (c) 2016 The Inter Project Authors | OFL-1.1, below |
| `packages/*/assets/fonts/jakarta-*.woff2` | [Plus Jakarta Sans](https://github.com/tokotype/PlusJakartaSans) — Copyright 2020 The Plus Jakarta Sans Project Authors | OFL-1.1, below |

Modifications: `vendor/tao/src/platform_impl/android/mod.rs` backports `MonitorHandle::size()`
from [tauri-apps/tao#1211](https://github.com/tauri-apps/tao/pull/1211); the rest of `vendor/tao`
is the crates.io release minus `examples/`. `vendor/jni-macros/src/types.rs` sorts the two loops
of `generate_type_mapping_checks` by type path; the rest of `vendor/jni-macros` is the crates.io
release minus `Cargo.lock` and `Cargo.toml.orig`. The OCR weights are the upstream `inference.onnx`
files, renamed `det.onnx` and `rec.onnx`, with `charset.txt` extracted from the recognition
model's configuration.

The licences of the Rust crates the web and mobile apps are built from are in
[packages/ui/licenses/third-party.txt](packages/ui/licenses/third-party.txt), which the apps show
at `/licenses`.

## SIL Open Font License 1.1

```text
SIL OPEN FONT LICENSE

Version 1.1 - 26 February 2007

PREAMBLE

The goals of the Open Font License (OFL) are to stimulate worldwide development of collaborative
font projects, to support the font creation efforts of academic and linguistic communities, and to
provide a free and open framework in which fonts may be shared and improved in partnership with
others.

The OFL allows the licensed fonts to be used, studied, modified and redistributed freely as long as
they are not sold by themselves. The fonts, including any derivative works, can be bundled,
embedded, redistributed and/or sold with any software provided that any reserved names are not used
by derivative works. The fonts and derivatives, however, cannot be released under any other type of
license. The requirement for fonts to remain under this license does not apply to any document
created using the fonts or their derivatives.

DEFINITIONS

"Font Software" refers to the set of files released by the Copyright Holder(s) under this license
and clearly marked as such. This may include source files, build scripts and documentation.

"Reserved Font Name" refers to any names specified as such after the copyright statement(s).

"Original Version" refers to the collection of Font Software components as distributed by the
Copyright Holder(s).

"Modified Version" refers to any derivative made by adding to, deleting, or substituting — in
part or in whole — any of the components of the Original Version, by changing formats or by
porting the Font Software to a new environment.

"Author" refers to any designer, engineer, programmer, technical writer or other person who
contributed to the Font Software.

PERMISSION & CONDITIONS

Permission is hereby granted, free of charge, to any person obtaining a copy of the Font Software,
to use, study, copy, merge, embed, modify, redistribute, and sell modified and unmodified copies of
the Font Software, subject to the following conditions:

1) Neither the Font Software nor any of its individual components, in Original or Modified
Versions, may be sold by itself.

2) Original or Modified Versions of the Font Software may be bundled, redistributed and/or sold
with any software, provided that each copy contains the above copyright notice and this license.
These can be included either as stand-alone text files, human-readable headers or in the
appropriate machine-readable metadata fields within text or binary files as long as those fields
can be easily viewed by the user.

3) No Modified Version of the Font Software may use the Reserved Font Name(s) unless explicit
written permission is granted by the corresponding Copyright Holder. This restriction only applies
to the primary font name as presented to the users.

4) The name(s) of the Copyright Holder(s) or the Author(s) of the Font Software shall not be used
to promote, endorse or advertise any Modified Version, except to acknowledge the contribution(s) of
the Copyright Holder(s) and the Author(s) or with their explicit written permission.

5) The Font Software, modified or unmodified, in part or in whole, must be distributed entirely
under this license, and must not be distributed under any other license. The requirement for fonts
to remain under this license does not apply to any document created using the Font Software.

TERMINATION

This license becomes null and void if any of the above conditions are not met.

DISCLAIMER

THE FONT SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
BUT NOT LIMITED TO ANY WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT OF COPYRIGHT, PATENT, TRADEMARK, OR OTHER RIGHT. IN NO EVENT SHALL THE COPYRIGHT
HOLDER BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, INCLUDING ANY GENERAL, SPECIAL,
INDIRECT, INCIDENTAL, OR CONSEQUENTIAL DAMAGES, WHETHER IN AN ACTION OF CONTRACT, TORT OR
OTHERWISE, ARISING FROM, OUT OF THE USE OR INABILITY TO USE THE FONT SOFTWARE OR FROM OTHER
DEALINGS IN THE FONT SOFTWARE.
```
