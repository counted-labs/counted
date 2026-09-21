# Regenerates every raster icon in the repo from assets/counted.svg.
#
# Run from the repo root, on Windows:  pwsh packages/mobile/gen-android-icons.ps1
# System.Drawing is Windows-only, so this does NOT run in the Linux devcontainer. The output is
# committed, so it only needs to run when the source artwork changes.
#
# The master is assets/counted.svg — the only artwork in the repo, rasterised here at 1024x1024 and
# scaled down for every output. Nothing is upscaled and nothing is recentred: the mark is drawn
# centred on its own 512 grid, so every output inherits that.
#
# Rasterising needs a Chromium browser (Edge ships with Windows). It renders the SVG inlined into a
# throwaway HTML page on a transparent ground; the white outputs composite that over white here.
#
# Outputs:
#   android-res/mipmap-*/ic_launcher_foreground.png   adaptive foreground, mark on transparency
#   android-res/mipmap-*/ic_launcher.png              legacy raster, mark on white
#   packages/{mobile,web}/assets/counted.png          in-app logo, 96x96
#   site/static/counted.png                           landing page logo + og:image
#   packages/mobile/assets/counted-1024.png           iOS AppIcon source, read by codemagic.yaml
#   packages/mobile/assets/counted-512.png            Play Store listing icon
#   packages/mobile/assets/play-feature-{fr,en}.png   Play Store feature graphic, 1024x500
#   packages/mobile/assets/counted.ico                mobile favicon asset
#   packages/desktop/assets/favicon.ico               desktop window icon

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"
Add-Type -AssemblyName System.Drawing

$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$res  = Join-Path $PSScriptRoot "android-res"
$svg  = Join-Path $PSScriptRoot "assets\counted.svg"

$WHITE = [System.Drawing.Color]::White
$MASTER_SIZE = 1024

# The adaptive foreground is drawn at 78/108 of the canvas. The mark is 416/512 tall on its own
# grid, so that lands its height at 63/108 — inside the 72dp safe zone the launcher's mask is
# guaranteed not to crop, whatever shape the OEM picked.
$SAFE_ZONE = 78 / 108

function New-Canvas([int]$size, [System.Drawing.Color]$fill) {
    $bmp = New-Object System.Drawing.Bitmap($size, $size, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.Clear($fill)
    $g.InterpolationMode  = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $g.PixelOffsetMode    = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
    $g.SmoothingMode      = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
    $g.CompositingQuality = [System.Drawing.Drawing2D.CompositingQuality]::HighQuality
    @{ Bitmap = $bmp; Graphics = $g }
}

# Draws $src into $g at ($x,$y) scaled to $size, clamping the edges. Without TileFlipXY the bicubic
# filter samples past the bitmap and leaves a translucent halo on the border pixels.
function Draw-Scaled($g, $src, [int]$x, [int]$y, [int]$size) {
    $attr = New-Object System.Drawing.Imaging.ImageAttributes
    $attr.SetWrapMode([System.Drawing.Drawing2D.WrapMode]::TileFlipXY)
    $dst = New-Object System.Drawing.Rectangle($x, $y, $size, $size)
    $g.DrawImage($src, $dst, 0, 0, $src.Width, $src.Height, [System.Drawing.GraphicsUnit]::Pixel, $attr)
    $attr.Dispose()
}

function Save-Png($bmp, [string]$path) {
    $dir = Split-Path -Parent $path
    if (-not (Test-Path $dir)) { New-Item -ItemType Directory $dir | Out-Null }
    $bmp.Save($path, [System.Drawing.Imaging.ImageFormat]::Png)
    "  {0,-46} {1}x{2}" -f (Resolve-Path -Relative $path), $bmp.Width, $bmp.Height
}

# A Vista-era .ico: ICONDIR, then one 16-byte ICONDIRENTRY per frame, then the frames themselves as
# whole PNG files. 0 in the width/height byte means 256.
function Save-Ico($src, [int[]]$sizes, [string]$path) {
    $frames = New-Object 'System.Collections.Generic.List[byte[]]'
    foreach ($s in $sizes) {
        $canvas = New-Canvas $s $WHITE
        Draw-Scaled $canvas.Graphics $src 0 0 $s
        $canvas.Graphics.Dispose()
        $ms = New-Object System.IO.MemoryStream
        $canvas.Bitmap.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
        $canvas.Bitmap.Dispose()
        $frames.Add($ms.ToArray())
        $ms.Dispose()
    }

    $out = New-Object System.IO.MemoryStream
    $w = New-Object System.IO.BinaryWriter($out)
    $w.Write([uint16]0); $w.Write([uint16]1); $w.Write([uint16]$sizes.Count)
    $offset = 6 + 16 * $sizes.Count
    for ($i = 0; $i -lt $sizes.Count; $i++) {
        $s = $sizes[$i]
        $w.Write([byte]($s % 256)); $w.Write([byte]($s % 256))
        $w.Write([byte]0); $w.Write([byte]0)
        $w.Write([uint16]1); $w.Write([uint16]32)
        $w.Write([uint32]$frames[$i].Length); $w.Write([uint32]$offset)
        $offset += $frames[$i].Length
    }
    foreach ($f in $frames) { $w.Write($f) }
    $w.Flush()
    [System.IO.File]::WriteAllBytes($path, $out.ToArray())
    $w.Dispose(); $out.Dispose()
    "  {0,-46} {1}" -f (Resolve-Path -Relative $path), ($sizes -join "/")
}

# --- 1. Rasterise the SVG ------------------------------------------------------------------------
# The SVG is inlined into the page rather than linked, so no file:// subresource has to be granted.
$browser = @(
    "${env:ProgramFiles(x86)}\Microsoft\Edge\Application\msedge.exe",
    "$env:ProgramFiles\Microsoft\Edge\Application\msedge.exe",
    "$env:ProgramFiles\Google\Chrome\Application\chrome.exe",
    "${env:ProgramFiles(x86)}\Google\Chrome\Application\chrome.exe"
) | Where-Object { Test-Path $_ } | Select-Object -First 1
if (-not $browser) { throw "no Edge or Chrome found — one is needed to rasterise assets/counted.svg" }

$tmp = Join-Path ([System.IO.Path]::GetTempPath()) ("counted-icons-" + [guid]::NewGuid())
New-Item -ItemType Directory $tmp | Out-Null
try {
    $page = Join-Path $tmp "master.html"
    $png  = Join-Path $tmp "master.png"
    $markup = Get-Content $svg -Raw
    Set-Content $page "<!doctype html><meta charset=`"utf-8`"><style>html,body{margin:0;padding:0;background:transparent}svg{display:block;width:100vw;height:100vh}</style>$markup" -Encoding utf8NoBOM

    & $browser --headless=new --disable-gpu --no-sandbox --hide-scrollbars `
        --user-data-dir="$tmp\profile" --default-background-color=00000000 `
        --screenshot="$png" --window-size="$MASTER_SIZE,$MASTER_SIZE" "file:///$($page -replace '\\','/')" | Out-Null
    if (-not (Test-Path $png)) { throw "the browser produced no screenshot — see $tmp" }

    $master = New-Object System.Drawing.Bitmap($png)
    if ($master.Width -ne $MASTER_SIZE) { throw "expected a ${MASTER_SIZE}x${MASTER_SIZE} raster, got $($master.Width)x$($master.Height)" }
    Write-Host "master: $($master.Width)x$($master.Height) rasterised from assets/counted.svg"

    # The same thing composited over white, for every output that is not the adaptive foreground.
    $wc = New-Canvas $MASTER_SIZE $WHITE
    $wc.Graphics.DrawImage($master, 0, 0, $MASTER_SIZE, $MASTER_SIZE)
    $onWhite = $wc.Bitmap
    $wc.Graphics.Dispose()

    # --- 2. Adaptive foregrounds: the mark alone, on transparency --------------------------------
    Write-Host "adaptive foregrounds (78/108 safe zone):"
    foreach ($d in @(
        @{ Dir = "mipmap-mdpi";    C = 108 },
        @{ Dir = "mipmap-hdpi";    C = 162 },
        @{ Dir = "mipmap-xhdpi";   C = 216 },
        @{ Dir = "mipmap-xxhdpi";  C = 324 },
        @{ Dir = "mipmap-xxxhdpi"; C = 432 }
    )) {
        $c = $d.C
        $s = [int][Math]::Round($c * $SAFE_ZONE)
        $canvas = New-Canvas $c ([System.Drawing.Color]::Transparent)
        Draw-Scaled $canvas.Graphics $master ([int](($c - $s) / 2)) ([int](($c - $s) / 2)) $s
        $canvas.Graphics.Dispose()
        Save-Png $canvas.Bitmap (Join-Path $res "$($d.Dir)\ic_launcher_foreground.png")
        $canvas.Bitmap.Dispose()
    }

    # --- 3. Legacy rasters (API 24-25, and android:icon pre-v26): the mark on a white square ------
    Write-Host "legacy rasters:"
    foreach ($d in @(
        @{ Dir = "mipmap-mdpi";    C = 48 },
        @{ Dir = "mipmap-hdpi";    C = 72 },
        @{ Dir = "mipmap-xhdpi";   C = 96 },
        @{ Dir = "mipmap-xxhdpi";  C = 144 },
        @{ Dir = "mipmap-xxxhdpi"; C = 192 }
    )) {
        $canvas = New-Canvas $d.C $WHITE
        Draw-Scaled $canvas.Graphics $onWhite 0 0 $d.C
        $canvas.Graphics.Dispose()
        Save-Png $canvas.Bitmap (Join-Path $res "$($d.Dir)\ic_launcher.png")
        $canvas.Bitmap.Dispose()
    }

    # --- 4. In-app logo: SplashScreen and the landing page render it at 32 CSS px, so 96 is exactly
    #        3x — the highest DPR that exists in practice. Larger is pure bytes for no visible gain.
    #        On transparency, not white: this file is also the web favicon, and a white square is
    #        visible around the mark in the Firefox tab strip.
    Write-Host "in-app logo:"
    $logo = New-Canvas 96 ([System.Drawing.Color]::Transparent)
    Draw-Scaled $logo.Graphics $master 0 0 96
    $logo.Graphics.Dispose()
    foreach ($p in @("packages\mobile\assets\counted.png", "packages\web\assets\counted.png", "site\static\counted.png")) {
        Save-Png $logo.Bitmap (Join-Path $root $p)
    }
    $logo.Bitmap.Dispose()

    # --- 5. iOS icon source ----------------------------------------------------------------------
    # codemagic.yaml's "Generate app icons" step scales every AppIcon size out of this one file with
    # `sips`, up to the 1024x1024 marketing icon. It used to read the 96px logo above, so that icon
    # was a 10.7x upscale. 24bpp on purpose: the App Store rejects an icon with an alpha channel,
    # and System.Drawing writes one into every 32bpp PNG even when it is fully opaque.
    Write-Host "iOS icon source:"
    $ios = New-Object System.Drawing.Bitmap(1024, 1024, [System.Drawing.Imaging.PixelFormat]::Format24bppRgb)
    $ig = [System.Drawing.Graphics]::FromImage($ios)
    $ig.Clear($WHITE)
    $ig.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $ig.PixelOffsetMode   = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
    Draw-Scaled $ig $onWhite 0 0 1024
    $ig.Dispose()
    Save-Png $ios (Join-Path $root "packages\mobile\assets\counted-1024.png")
    $ios.Dispose()

    # --- 6. Play Store listing icon: 512x512, the mark on white -----------------------------------
    Write-Host "Play Store icon:"
    $play = New-Canvas 512 $WHITE
    Draw-Scaled $play.Graphics $onWhite 0 0 512
    $play.Graphics.Dispose()
    Save-Png $play.Bitmap (Join-Path $root "packages\mobile\assets\counted-512.png")
    $play.Bitmap.Dispose()

    # --- 7. Window icons -------------------------------------------------------------------------
    Write-Host "window icons:"
    Save-Ico $onWhite @(16, 32, 48, 64, 128, 256) (Join-Path $root "packages\mobile\assets\counted.ico")
    Save-Ico $onWhite @(16, 32, 48, 64, 128, 256) (Join-Path $root "packages\desktop\assets\favicon.ico")

    $onWhite.Dispose()
    $master.Dispose()

    # --- 8. Play Store feature graphic ---------------------------------------------------------------
    # Rendered by the browser like the master, one page per listing language. Play rounds the corners
    # and crops the edges in some placements, so everything sits inside a 64px margin. The fonts are
    # the landing page's own, inlined since a file:// page cannot load file:// subresources.
    Write-Host "Play Store feature graphics:"
    $fonts = @{ jakarta = "jakarta-700.woff2"; inter = "inter-500.woff2" }.GetEnumerator() | ForEach-Object {
        $b64 = [Convert]::ToBase64String([IO.File]::ReadAllBytes((Join-Path $root "site\static\fonts\$($_.Value)")))
        $family = if ($_.Key -eq "jakarta") { "Plus Jakarta Sans" } else { "Inter" }
        "@font-face{font-family:'$family';src:url(data:font/woff2;base64,$b64) format('woff2')}"
    }
    foreach ($l in @(
        @{ Lang = "fr"; Title = "Vos comptes ne regardent<br>que vous."; Sub = "Partagez vos dépenses entre amis. Chiffré sur votre appareil." },
        @{ Lang = "en"; Title = "Your books are nobody<br>else&rsquo;s business."; Sub = "Split expenses with friends. Encrypted on your device." }
    )) {
        $fpage = Join-Path $tmp "feature-$($l.Lang).html"
        $fpng  = Join-Path $tmp "feature-$($l.Lang).png"
        Set-Content $fpage @"
<!doctype html><meta charset="utf-8"><style>
$($fonts -join "`n")
html,body{margin:0;padding:0}
body{box-sizing:border-box;width:1024px;height:500px;padding:0 64px;background:oklch(97.5% 0.004 165);color:oklch(25% 0.02 165);display:flex;align-items:center;justify-content:center;gap:48px;font-family:Inter,sans-serif}
svg{width:260px;height:260px;flex:none}
h1{margin:0;font:700 46px/1.15 'Plus Jakarta Sans',sans-serif;letter-spacing:-0.02em;white-space:nowrap}
p{margin:16px 0 0;font-size:22px;line-height:1.35;color:oklch(25% 0.02 165 / 0.65);max-width:540px}
</style><body>$markup<div><h1>$($l.Title)</h1><p>$($l.Sub)</p></div>
"@ -Encoding utf8NoBOM

        & $browser --headless=new --disable-gpu --no-sandbox --hide-scrollbars `
            --user-data-dir="$tmp\profile" `
            --screenshot="$fpng" --window-size="1024,500" "file:///$($fpage -replace '\\','/')" | Out-Null
        if (-not (Test-Path $fpng)) { throw "the browser produced no feature graphic — see $tmp" }

        $shot = New-Object System.Drawing.Bitmap($fpng)
        $feature = New-Object System.Drawing.Bitmap(1024, 500, [System.Drawing.Imaging.PixelFormat]::Format24bppRgb)
        $fg = [System.Drawing.Graphics]::FromImage($feature)
        $fg.DrawImage($shot, 0, 0, 1024, 500)
        $fg.Dispose()
        $shot.Dispose()
        Save-Png $feature (Join-Path $root "packages\mobile\assets\play-feature-$($l.Lang).png")
        $feature.Dispose()
    }
} finally {
    Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
}

Write-Host "done."
