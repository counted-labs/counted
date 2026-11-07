#!/bin/sh
# Pins the web bundle to the HTML document: the wasm by a sha384 the loader fetches it with, the
# loader by `integrity` on its <script> and preload. Run on public/ after `dx bundle`, before the
# sums, by Dockerfile and Dockerfile.client alike. docs/dioxus-overrides.md section 17.
set -eu
cd "${1:?usage: integrity.sh <bundle public dir>}"

sri() { printf 'sha384-%s' "$(openssl dgst -sha384 -binary "$1" | base64 -w0)"; }
one() { [ "$(printf '%s\n' "$2" | grep -c .)" = 1 ] || { echo "integrity.sh: expected one $1, found: $2" >&2; exit 1; }; }

wasm=$(find assets -maxdepth 1 -name '*.wasm')
one wasm "$wasm"
js=$(grep -l -F "module_or_path:\"/./$wasm\"" assets/*.js || true)
one "loader naming /./$wasm" "$js"
[ "$(grep -o -F "module_or_path:\"/./$wasm\"" "$js" | wc -l)" = 1 ] || { echo "integrity.sh: $js loads the wasm more than once" >&2; exit 1; }

sed -i "s|module_or_path:\"/\./$wasm\"|module_or_path:fetch(\"/./$wasm\",{integrity:\"$(sri "$wasm")\"})|" "$js"
# The brotli sibling still holds the unpinned loader, and the server would prefer it.
rm -f "$js.br"

js_sri=$(sri "$js")
sed -i -e "s|<script type=\"module\" async src=\"/\./$js\">|<script type=\"module\" async src=\"/./$js\" integrity=\"$js_sri\">|" \
	-e "s|<link rel=\"preload\" as=\"script\" href=\"/\./$js\" crossorigin>|<link rel=\"preload\" as=\"script\" href=\"/./$js\" crossorigin integrity=\"$js_sri\">|" \
	index.html
[ "$(grep -o "<script[^>]*src=" index.html | wc -l)" = "$(grep -o "<script[^>]*src=[^>]*integrity=\"sha384-" index.html | wc -l)" ] \
	|| { echo "integrity.sh: a <script src> in index.html has no integrity" >&2; exit 1; }
[ "$(grep -o "<link[^>]*as=\"script\"" index.html | wc -l)" = "$(grep -o "<link[^>]*as=\"script\"[^>]*integrity=\"sha384-" index.html | wc -l)" ] \
	|| { echo "integrity.sh: a script preload in index.html has no integrity" >&2; exit 1; }
grep -q "integrity=\"$js_sri\"" index.html || { echo "integrity.sh: index.html does not load $js" >&2; exit 1; }
