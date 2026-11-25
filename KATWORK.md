# KatWork fork of FilmCraft

This repository is the KatWork fork of FilmCraft (https://github.com/storytold/filmcraft). KatWork builds the app's CLI and web
build from this fork. It carries the patches listed below; everything else is upstream code.

Based on FilmCraft by the ArtCraft team (https://github.com/storytold/filmcraft). This build is not an ArtCraft product.

## Fork base

- Fork base (this fork's `main` when the KatWork work started): `740adb32cc38a9c685ef531c2d6a8e57cde66203`
- Upstream `storytold/filmcraft` head at that time: `740adb32cc38a9c685ef531c2d6a8e57cde66203`

## KatWork patches

| Patch | Upstream PR |
|---|---|
| `chore(katwork): remove the ArtCraft marks` | not proposed upstream (the brand terms let a fork that proposes changes keep the marks; this one is a distributed modified version) |
| `feat(katwork): cargo xtask web forwards Cargo features` | draft, not opened (in the fork PR body) |

## The marks

The ArtCraft name, wordmark and logos are trademarks of the ArtCraft Team and are not covered by the app's licence. This fork
removes them, as the brand terms require of a distributed modified version: `docs/brand/` is deleted; links to the ArtCraft
website and its community (Discord), and taglines that present the build as an ArtCraft product, are removed; help links and
`app.links`-style results point to the upstream repository (https://github.com/storytold/filmcraft); README.md, NOTICE and the
About window carry the "Based on" line above as plain text. No new logo is drawn: where a mark was shown, the app's name is shown
as plain text.

`scripts/check-no-marks.sh` (CI job `no-marks` in `.github/workflows/katwork.yml`) fails when a tracked file is a copy of a
fork-base brand file (`scripts/katwork-marks/hashes`), when a line names the marks or the ArtCraft website, when a line names
ArtCraft and is not in `scripts/katwork-marks/allow`, or when an image below is missing from this file.

### What still names ArtCraft, and why (`scripts/katwork-marks/allow`)

- Copyright notices (`LICENSE-MIT`, `NOTICE`, README): required by the licences.
- Application ids `ai.storyteller.filmcraft` and installer paths and folders (`Programs\ArtCraft\...`): kept unchanged; they are
  identifiers, not marks, and changing them would break upgrades of existing installs.
- The Windows install-policy check's case for ArtCraft Launcher installs and the Flatpak permission for the launcher's version
  folder (`xdg-data/ArtCraft/versions`): they name a separate program and its folders, the upstream nightly builds' updater;
  KatWork builds no nightly.
- The upstream release workflow's secret name (`release.yml`): upstream CI configuration, unchanged; KatWork does not run it.
- The "Based on" line in the About dialog's code (`crates/ui-egui/src/brand.rs`).

### Other text this fork changed

- The About dialog and the Home (Import) screen drew the ArtCraft wordmark, embedded from `docs/brand/` with `include_bytes!`.
  The About dialog's own "FilmCraft" heading now heads the window in its place; on the Home screen the app's name, as plain
  text at the section titles' size and weight, takes the wordmark's place and the section's "Community" title.
- The header's Discord button, the Help menu's Discord, ArtCraft website and getartcraft.com items (`help.discord`,
  `help.website`, `help.appPage`) and the macOS app menu's Discord item are removed; the About dialog and the Home screen link
  to the GitHub repository and its issues. The About tagline loses "Part of the ArtCraft family." in English and in every
  catalog.
- The issue templates' and `SECURITY.md`'s Discord links point to the GitHub issues instead; the asset sidecars,
  `ATTRIBUTION.md` and `assets/app-icon/` said the app icon was drawn "in ArtCraft" and now say it was drawn by the owner
  (author, source and licence unchanged); docs that called the feature naming scheme "ArtCraft" no longer do.

## Image assets of the fork base

Removed (the brand files, deleted with `docs/brand/`):

- `docs/brand/LICENSE-brand.txt`: removed
- `docs/brand/artcraft-logo-white.png`: removed
- `docs/brand/artcraft-logo-white.png.attribution`: removed
- `docs/brand/artcraft-logo-white.svg`: removed
- `docs/brand/artcraft-logo-white.svg.attribution`: removed
- `docs/brand/artcraft-logo.png`: removed
- `docs/brand/artcraft-logo.png.attribution`: removed
- `docs/brand/artcraft-logo.svg`: removed
- `docs/brand/artcraft-logo.svg.attribution`: removed
- `docs/brand/artcraft-mark-black.png`: removed
- `docs/brand/artcraft-mark-black.png.attribution`: removed
- `docs/brand/artcraft-mark-black.svg`: removed
- `docs/brand/artcraft-mark-black.svg.attribution`: removed
- `docs/brand/artcraft-mark.png`: removed
- `docs/brand/artcraft-mark.png.attribution`: removed
- `docs/brand/artcraft-mark.svg`: removed
- `docs/brand/artcraft-mark.svg.attribution`: removed

Every other image of the fork base, each kept:

- `apps/filmcraft-web/web/favicon.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/filmcraft-1024.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/filmcraft-macos-512.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/filmcraft-small.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/filmcraft.icns`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/filmcraft.ico`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/filmcraft.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/128x128/apps/ai.storyteller.filmcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/16x16/apps/ai.storyteller.filmcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/24x24/apps/ai.storyteller.filmcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/256x256/apps/ai.storyteller.filmcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/32x32/apps/ai.storyteller.filmcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/48x48/apps/ai.storyteller.filmcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/512x512/apps/ai.storyteller.filmcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/64x64/apps/ai.storyteller.filmcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/scalable/apps/ai.storyteller.filmcraft.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `crates/codecs/tests/fixtures/mjpeg-avi1-baseline.jpg`: kept: codec test fixture (viewed: no ArtCraft mark)
- `crates/codecs/tests/fixtures/mjpeg-cmyk.jpg`: kept: codec test fixture (viewed: no ArtCraft mark)
- `crates/codecs/tests/fixtures/mjpeg-progressive-huffman.jpg`: kept: codec test fixture (viewed: no ArtCraft mark)
- `crates/golden/goldens/blend_modes.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/crop.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/gaussian_blur.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/graphic_rich_text.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/graphic_shapes.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/graphic_template.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/graphic_title.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/hdr_tone_map.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/log_to_rec709.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/lumetri_basic.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/lumetri_hdr_pq.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/lumetri_presets_grid.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/mask_blur.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/mask_color.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/mask_inverted.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/mask_opacity.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/text_burnin.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/transform_opacity.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/transition_cross_dissolve.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/transition_dip_to_black.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/transition_wipe.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/vfx_distort.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/vfx_glows.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/vfx_spacer_stroke_shadow.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/vfx_stylize.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/vfx_ultra_key.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/golden/goldens/vfx_vr_rotate.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `docs/images/filmcraft-assembly.png`: kept: screenshot of the upstream build (viewed, header bar included: no ArtCraft mark, no Discord button)
- `docs/images/filmcraft-color-panel.png`: kept: screenshot of the upstream build (viewed, header bar included: no ArtCraft mark, no Discord button)
- `docs/images/filmcraft-color.png`: kept: screenshot of the upstream build (viewed, header bar included: no ArtCraft mark, no Discord button)
- `docs/images/filmcraft-effects.png`: kept: screenshot of the upstream build (viewed, header bar included: no ArtCraft mark, no Discord button)
- `docs/images/filmcraft-export.png`: kept: screenshot of the upstream build (viewed, header bar included: no ArtCraft mark, no Discord button)
- `docs/images/filmcraft-hero.png`: kept: screenshot of the upstream build (viewed, header bar included: no ArtCraft mark, no Discord button)
- `docs/images/filmcraft-keyframes.png`: kept: screenshot of the upstream build (viewed, header bar included: no ArtCraft mark, no Discord button)
- `docs/images/filmcraft-scopes.png`: kept: screenshot of the upstream build (viewed, header bar included: no ArtCraft mark, no Discord button)
- `docs/images/filmcraft-timeline.png`: kept: screenshot of the upstream build (viewed, header bar included: no ArtCraft mark, no Discord button)
- `packaging/macos/dmg/background.svg`: kept: macOS disk-image background, original artwork (the app name and icon; viewed as its TIFF rendering: no ArtCraft mark)
- `packaging/macos/dmg/background.tiff`: kept: macOS disk-image background, original artwork (the app name and icon; viewed: no ArtCraft mark)
