# Android build rules. The private Makefile includes this file; the public mirror has only this
# file, and F-Droid runs it standalone from the repository root:
#   make -f packages/mobile/android.mk android-fdroid-apk
# So nothing here may need a keystore, the Play API or a device. See docs/android.md, "F-Droid build".
ANDROID_MK := $(lastword $(MAKEFILE_LIST))
ANDROID_MK_DEFAULT_GOAL := $(.DEFAULT_GOAL)

.PHONY: android-apk-unsigned android-fdroid-apk

# Same trap as SSH_KEY in the Makefile, opposite fix. A recipe's shell eats the backslashes in make's
# $(HOME), so $(HOME)/x arrives as "C:Usersjbosix" and cargo creates it relative to the repo.
# $$HOME is wrong here too: it yields a POSIX path, which cargo (a Windows binary) misreads.
HOME_FS := $(subst \,/,$(HOME))

# Android builds must live on a Linux filesystem so dx can chmod gradlew.
# On WSL/devcontainer the project is on NTFS (bind mount) which blocks chmod.
ANDROID_BUILD_DIR ?= $(HOME_FS)/.counted-android-build

# Push (patch_push). 1 bundles the embedded FCM distributor (Play build); the F-Droid build passes 0.
PUSH_FCM_FALLBACK ?= 1
UNIFIEDPUSH_CONNECTOR    := 3.3.5
UNIFIEDPUSH_EMBEDDED_FCM := 3.1.0
KOTLIN_GRADLE_PLUGIN     := 2.2.21
# Must match ANDROID_NDK_VERSION in .devcontainer/dockerFile and NDK_VERSION in
# libraries/docker/android/Dockerfile — AGP 8.7 otherwise defaults to an NDK
# that is not installed, and the debug-symbol extraction in android-release fails.
NDK_VERSION   ?= 25.2.9519653
# Both spellings: the SDK ships `aapt2` on Linux and only `aapt2.exe` on Windows.
AAPT2         ?= $(shell find "$${ANDROID_HOME:-$(HOME_FS)/Android/Sdk}/build-tools" \( -name aapt2 -o -name aapt2.exe \) 2>/dev/null | sort -V | tail -1)

# Keeps the E2EE keys off Google's backup servers. $(1) is the .../android directory.
#
# counted_local_storage.json holds the account key *and* every per-project encryption key, next to
# the cached ciphertext — so an auto-backup is self-contained: keys and data in one archive, and a
# project key can never be rotated. iOS has always excluded its data directory
# (packages/mobile/src/main.rs sets NSURLIsExcludedFromBackupKey, with that reason written out);
# Android inherited allowBackup's default of true, so the stated threat model was enforced on
# exactly one of the two platforms.
#
# Both attributes are needed: allowBackup covers pre-12 devices and adb backup, and
# dataExtractionRules covers Android 12+ cloud backup and device-to-device transfer, which
# allowBackup alone no longer governs. The greps are the gate, as with the deep-link patches.
#
# The resource is copied here rather than left to the `cp -r android-res/.` the release targets do,
# because `e2e-android-apk` calls the patches without that copy — self-contained means every path
# that patches the manifest also ships the file it references.
define patch_no_backup
	mkdir -p $(1)/app/app/src/main/res/xml
	cp packages/mobile/android-res/xml/data_extraction_rules.xml \
		$(1)/app/app/src/main/res/xml/data_extraction_rules.xml
	sed -i 's|<application |<application android:allowBackup="false" android:dataExtractionRules="@xml/data_extraction_rules" |' \
		$(1)/app/app/src/main/AndroidManifest.xml
	@grep -q 'android:allowBackup="false"' $(1)/app/app/src/main/AndroidManifest.xml \
		|| { echo "ERROR: allowBackup patch did not apply — dx's manifest template changed"; exit 1; }
	@grep -q 'android:dataExtractionRules' $(1)/app/app/src/main/AndroidManifest.xml \
		|| { echo "ERROR: dataExtractionRules patch did not apply — dx's manifest template changed"; exit 1; }
endef

# targetSdk 36 turns predictive back on by default, and the system then stops dispatching
# KEYCODE_BACK to the activity. wry's WryActivity handles back in `onKeyDown` (it calls
# WebView.goBack(), which is what our `_dioxusBackArmed` bridge listens for), so with the callback
# enabled every back press would close the app instead of navigating. See docs/mobile-navigation.md.
define patch_predictive_back
	sed -i 's|<application |<application android:enableOnBackInvokedCallback="false" |' \
		$(1)/app/app/src/main/AndroidManifest.xml
	@grep -q 'android:enableOnBackInvokedCallback="false"' $(1)/app/app/src/main/AndroidManifest.xml \
		|| { echo "ERROR: predictive-back opt-out did not apply — dx's manifest template changed"; exit 1; }
endef

# The FileProvider the receipt-scan camera needs. $(1) is the .../android directory.
#
# wry 0.53.5's RustWebChromeClient.onShowFileChooser already implements capture properly: with
# isCaptureEnabled and accept="image/*" it launches MediaStore.ACTION_IMAGE_CAPTURE and writes
# through FileProvider.getUriForFile(activity, packageName + ".fileprovider", ...).
#
# dx 0.7.9's AndroidManifest.xml.hbs declares NO <provider>. That call throws, wry swallows it in
# `catch (ex: Exception)`, showImageCapturePicker returns false, and it falls back to the document
# picker. No error is logged anywhere — the Scan button silently opens the gallery instead of the
# camera, which is why the greps below are the gate.
#
# Do NOT add android.permission.CAMERA. wry's isMediaCaptureSupported returns true when the
# permission is not *defined* in the manifest, so capture proceeds with no runtime prompt;
# ACTION_IMAGE_CAPTURE does not need it (the camera app takes the photo), and declaring it buys a
# permission dialog and a Play listing entry for nothing.
#
# androidx.core is already on the classpath through appcompat in dx's build.gradle.kts.hbs — wry's
# own Kotlin imports androidx.core.content.FileProvider and compiles today. The authority is
# applicationId + ".fileprovider", and applicationId comes from [bundle] identifier in
# packages/mobile/Dioxus.toml.
#
# The resource is copied here rather than left to `cp -r android-res/.`, for the same reason
# patch_no_backup does it: e2e-android-apk calls the patches without that copy.
define patch_file_provider
	mkdir -p $(1)/app/app/src/main/res/xml
	cp packages/mobile/android-res/xml/file_paths.xml \
		$(1)/app/app/src/main/res/xml/file_paths.xml
	sed -i 's|</application>|\t<provider android:name="androidx.core.content.FileProvider" android:authorities="fr.counted.app.fileprovider" android:exported="false" android:grantUriPermissions="true">\n\t\t\t<meta-data android:name="android.support.FILE_PROVIDER_PATHS" android:resource="@xml/file_paths" />\n\t\t</provider>\n\t</application>|' \
		$(1)/app/app/src/main/AndroidManifest.xml
	@grep -q 'fr.counted.app.fileprovider' $(1)/app/app/src/main/AndroidManifest.xml \
		|| { echo "ERROR: FileProvider patch did not apply — dx's manifest template changed. The Scan button will silently open the gallery instead of the camera."; exit 1; }
	@grep -q '@xml/file_paths' $(1)/app/app/src/main/AndroidManifest.xml \
		|| { echo "ERROR: FILE_PROVIDER_PATHS meta-data missing — FileProvider.getUriForFile will throw"; exit 1; }
endef

# The greps above read the *source* manifest, which cannot see a manifest-merger drop; the binary
# manifest inside the APK is the artifact that actually ships. $(1) is the built APK.
#
# Worth gating rather than trusting: with no provider, wry's FileProvider.getUriForFile throws,
# RustWebChromeClient swallows it in `catch (ex: Exception)` and falls back to the document picker,
# so the failure ships as "Scan opens the gallery" with nothing but a `Tauri/FileChooser` log line.
define check_camera_capture_apk
	@test -n "$(AAPT2)" \
		|| { echo "ERROR: aapt2 not found under \$$ANDROID_HOME/build-tools — cannot verify the compiled manifest"; exit 1; }
	@"$(AAPT2)" dump xmltree $(1) --file AndroidManifest.xml | grep -q 'fr.counted.app.fileprovider' \
		|| { echo "ERROR: FileProvider absent from the compiled manifest in $(1) — the Scan button will silently open the gallery instead of the camera"; exit 1; }
	@# The other half, and it fails the same silent way: wry calls resolveActivity() before
	@# launching the camera, and on targetSdk >= 30 that returns null without this <queries>
	@# declaration. It comes from [android.raw] manifest in packages/mobile/Dioxus.toml.
	@"$(AAPT2)" dump xmltree $(1) --file AndroidManifest.xml | grep -q 'android.media.action.IMAGE_CAPTURE' \
		|| { echo "ERROR: no <queries> for IMAGE_CAPTURE in $(1) — resolveActivity returns null on targetSdk >= 30, so Scan opens the gallery"; exit 1; }
endef

# Push notifications (UnifiedPush / Web Push). $(1) is the .../android directory. See
# docs/plans/push-notifications.md.
#
# Three pieces dx's template has none of: the Kotlin PushService and the <service> that names it,
# the POST_NOTIFICATIONS permission (a runtime prompt on 13+, but the manifest must declare it),
# and the UnifiedPush connector MainActivity.kt imports. No Google code: the connector's only
# dependency is Tink (Apache-2.0), and no Firebase project or google-services.json exists.
#
# PUSH_FCM_FALLBACK=1 adds the embedded FCM distributor, which serves phones that have Play
# Services (or microG) but no distributor app — the stock case, and the Play Store build. It is
# FOSS (no Firebase SDK: it registers through the Play Services c2dm intent), but it talks to a
# Google service, so the F-Droid build leaves it out (PUSH_FCM_FALLBACK=0), as Fedilab does; its
# users bring ntfy, Sunup or the UP-FCM distributor app. Its receivers come from the library's own
# manifest, so only the dependency is patched.
#
# Both libraries pull kotlin-stdlib 2.2/2.3, whose metadata dx 0.7.9's Kotlin 2.0.20 compiler
# refuses ("incompatible version of Kotlin"), so the Kotlin plugin is bumped to 2.2 — which reads
# one version ahead. Not 2.3: it turns the template's `kotlinOptions { jvmTarget }` into an error.
#
# ic_notification is copied here for the same reason patch_no_backup and patch_file_provider copy
# theirs: the Kotlin above names R.drawable.ic_notification, and e2e-android-apk calls the patches
# without the `cp -r android-res/.` the release targets do — so without this the e2e APK fails to
# compile on an unresolved reference.
define patch_push
	mkdir -p $(1)/app/app/src/main/res/drawable
	cp packages/mobile/android-res/drawable/ic_notification.xml \
		$(1)/app/app/src/main/res/drawable/ic_notification.xml
	cp packages/mobile/android-kotlin/CountedPushService.kt \
		$(1)/app/app/src/main/kotlin/dev/dioxus/main/CountedPushService.kt
	sed -i 's|<uses-permission android:name="android.permission.INTERNET" />|<uses-permission android:name="android.permission.INTERNET" />\n    <uses-permission android:name="android.permission.POST_NOTIFICATIONS" />|' \
		$(1)/app/app/src/main/AndroidManifest.xml
	sed -i 's|</application>|\t<service android:name="dev.dioxus.main.CountedPushService" android:exported="false">\n\t\t\t<intent-filter>\n\t\t\t\t<action android:name="org.unifiedpush.android.connector.PUSH_EVENT" />\n\t\t\t</intent-filter>\n\t\t</service>\n\t</application>|' \
		$(1)/app/app/src/main/AndroidManifest.xml
	sed -i 's|implementation("androidx.webkit:webkit:1.13.0")|implementation("androidx.webkit:webkit:1.13.0")\n    implementation("org.unifiedpush.android:connector:$(UNIFIEDPUSH_CONNECTOR)")|' \
		$(1)/app/app/build.gradle.kts
	sed -i 's|kotlin-gradle-plugin:2.0.20|kotlin-gradle-plugin:$(KOTLIN_GRADLE_PLUGIN)|' $(1)/app/build.gradle.kts
	@grep -q 'kotlin-gradle-plugin:$(KOTLIN_GRADLE_PLUGIN)' $(1)/app/build.gradle.kts \
		|| { echo "ERROR: Kotlin plugin bump did not apply — dx's root build.gradle.kts template changed"; exit 1; }
	@grep -q 'POST_NOTIFICATIONS' $(1)/app/app/src/main/AndroidManifest.xml \
		|| { echo "ERROR: POST_NOTIFICATIONS patch did not apply — dx's manifest template changed"; exit 1; }
	@grep -q 'org.unifiedpush.android.connector.PUSH_EVENT' $(1)/app/app/src/main/AndroidManifest.xml \
		|| { echo "ERROR: push <service> patch did not apply — dx's manifest template changed"; exit 1; }
	@grep -q 'org.unifiedpush.android:connector' $(1)/app/app/build.gradle.kts \
		|| { echo "ERROR: UnifiedPush connector dependency patch did not apply — dx's build.gradle.kts template changed"; exit 1; }
	@if test "$(PUSH_FCM_FALLBACK)" = 1; then \
		sed -i 's|implementation("org.unifiedpush.android:connector:$(UNIFIEDPUSH_CONNECTOR)")|implementation("org.unifiedpush.android:connector:$(UNIFIEDPUSH_CONNECTOR)")\n    implementation("org.unifiedpush.android:embedded-fcm-distributor:$(UNIFIEDPUSH_EMBEDDED_FCM)")|' \
			$(1)/app/app/build.gradle.kts; \
		grep -q 'embedded-fcm-distributor' $(1)/app/app/build.gradle.kts \
			|| { echo "ERROR: embedded FCM distributor patch did not apply"; exit 1; }; \
	else \
		echo "push: no embedded FCM distributor (PUSH_FCM_FALLBACK=$(PUSH_FCM_FALLBACK)) — devices need a UnifiedPush distributor app"; \
	fi
endef

# Deep-link patches applied to the generated Android project. $(1) is the .../android directory.
#
# dx 0.7.9 renders the intent-filters from [deep_links] in packages/mobile/Dioxus.toml, but its
# manifest template sets no launchMode and its MainActivity.kt has no body. Without singleTask a
# VIEW intent spawns a second NativeActivity (re-initialising ndk_context); without the
# onNewIntent override getIntent() keeps returning the launch intent, so a link tapped while the
# app is running would raise it without navigating. See docs/deep-links.md.
#
# The greps are the gate: a dx upgrade that renames the activity or drops the [deep_links]
# rendering must fail the build here rather than ship an app that silently stops handling links.
define patch_deep_links
	sed -i 's|android:name="dev.dioxus.main.MainActivity"|android:name="dev.dioxus.main.MainActivity" android:launchMode="singleTask"|' \
		$(1)/app/app/src/main/AndroidManifest.xml
	cp packages/mobile/android-kotlin/MainActivity.kt \
		$(1)/app/app/src/main/kotlin/dev/dioxus/main/MainActivity.kt
	$(call patch_no_backup,$(1))
	$(call patch_predictive_back,$(1))
	$(call patch_file_provider,$(1))
	$(call patch_push,$(1))
	@grep -q 'android:launchMode="singleTask"' $(1)/app/app/src/main/AndroidManifest.xml \
		|| { echo "ERROR: launchMode patch did not apply — dx's manifest template changed"; exit 1; }
	@grep -q 'android.intent.action.VIEW' $(1)/app/app/src/main/AndroidManifest.xml \
		|| { echo "ERROR: no deep-link intent-filter — check [deep_links] in packages/mobile/Dioxus.toml"; exit 1; }
	@grep -q 'onNewIntent' $(1)/app/app/src/main/kotlin/dev/dioxus/main/MainActivity.kt \
		|| { echo "ERROR: MainActivity.kt override not copied"; exit 1; }
endef

# Lets AGP strip the release .so. $(1) is the .../android directory.
#
# dx's build.gradle.kts.hbs puts `packaging { jniLibs.keepDebugSymbols.add(...) }` inside
# `getByName("debug")`, but a build type has no `packaging` of its own: the Kotlin DSL resolves it to
# the enclosing `android {}`, so it applies to release too. AGP then packages the .so as-is, and its
# SYMBOL_TABLE extraction — which only runs when stripping changed the file's size — silently skips.
# [profile.android-release] in Cargo.toml is the other half: dx must hand Gradle an unstripped .so.
define patch_release_strip
	sed -i '/jniLibs.keepDebugSymbols.add/d' $(1)/app/app/build.gradle.kts
	@! grep -q 'keepDebugSymbols' $(1)/app/app/build.gradle.kts \
		|| { echo "ERROR: keepDebugSymbols still in build.gradle.kts — dx's template changed, release .so would ship unstripped"; exit 1; }
	@grep -q 'ndkVersion' $(1)/app/app/build.gradle.kts \
		|| { echo "ERROR: ndkVersion not pinned — AGP 8.7 defaults to an NDK that is not installed"; exit 1; }
endef

# What F-Droid builds and the android-fdroid CI job signs: no embedded FCM distributor, and the
# versionCode committed in packages/mobile/Cargo.toml (major*1000000 + minor*1000 + patch of
# `version`), since F-Droid cannot ask the Play API. The check catches a `version` bump without it.
android-fdroid-apk:
	@code=$$(sed -n 's/^version_code = \([0-9][0-9]*\)$$/\1/p' packages/mobile/Cargo.toml); \
	version=$$(sed -n 's/^version = "\([0-9.]*\)"$$/\1/p' packages/mobile/Cargo.toml | head -1); \
	expected=$$(echo "$$version" | awk -F. '{ print $$1 * 1000000 + $$2 * 1000 + $$3 }'); \
	test -n "$$code" && test "$$code" = "$$expected" \
		|| { echo "ERROR: version_code in packages/mobile/Cargo.toml is '$$code', version $$version needs $$expected"; exit 1; }; \
	$(MAKE) --no-print-directory -f $(ANDROID_MK) android-apk-unsigned VERSION_CODE=$$code PUSH_FCM_FALLBACK=0

# android-release-apk up to signing. A copy rather than a shared recipe, so the Play build is untouched.
android-apk-unsigned:
	@echo "$(VERSION_CODE)" | grep -qE '^[0-9]+$$' || { echo "ERROR: VERSION_CODE must be an integer — pass VERSION_CODE=<n>"; exit 1; }
	rm -rf $(ANDROID_BUILD_DIR)/dx/mobile/release/android/
	@# NO_DOWNLOADS: an F-Droid build must not fetch tools. dx takes any it needs from PATH or fails.
	@# COUNTED_STORE_URL: the update-required screen links here instead of the Play Store.
	CARGO_TARGET_DIR=$(ANDROID_BUILD_DIR) NO_DOWNLOADS=1 COUNTED_STORE_URL=https://f-droid.org/packages/fr.counted.app/ \
		dx bundle --platform android --package mobile --release --target aarch64-linux-android
	sed -i 's|<string name="app_name">Mobile</string>|<string name="app_name">Counted</string>|' \
		$(ANDROID_BUILD_DIR)/dx/mobile/release/android/app/app/src/main/res/values/strings.xml
	find $(ANDROID_BUILD_DIR)/dx/mobile/release/android/app/app/src/main/res \
		-name "ic_launcher*.webp" -delete
	cp -r packages/mobile/android-res/. \
		$(ANDROID_BUILD_DIR)/dx/mobile/release/android/app/app/src/main/res/
	sed -i "s/versionCode = [0-9]*/versionCode = $(VERSION_CODE)/" \
		$(ANDROID_BUILD_DIR)/dx/mobile/release/android/app/app/build.gradle.kts
	sed -i "s/targetSdk = [0-9]*/targetSdk = 36/" \
		$(ANDROID_BUILD_DIR)/dx/mobile/release/android/app/app/build.gradle.kts
	sed -i "s/compileSdk = [0-9]*/compileSdk = 36\n    ndkVersion = \"$(NDK_VERSION)\"/" \
		$(ANDROID_BUILD_DIR)/dx/mobile/release/android/app/app/build.gradle.kts
	$(call patch_release_strip,$(ANDROID_BUILD_DIR)/dx/mobile/release/android)
	$(call patch_deep_links,$(ANDROID_BUILD_DIR)/dx/mobile/release/android)
	cd $(ANDROID_BUILD_DIR)/dx/mobile/release/android/app && ./gradlew assembleRelease
	cp $(ANDROID_BUILD_DIR)/dx/mobile/release/android/app/app/build/outputs/apk/release/app-release-unsigned.apk \
		Counted-unsigned.apk
	$(call check_camera_capture_apk,Counted-unsigned.apk)
	@echo "APK ready: Counted-unsigned.apk"

.DEFAULT_GOAL := $(ANDROID_MK_DEFAULT_GOAL)
