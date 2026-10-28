# mobile

The Android and iOS apps: the `ui` crate rendered in the system WebView. `android-kotlin/`,
`android-res/` and `android-gradle/` are copied over what `dx` generates, and `ios/` holds the iOS
`Info.plist` and localized strings. `android.mk` is the Android build, runnable on its own - the
F-Droid recipe uses `make -f packages/mobile/android.mk android-fdroid-apk`.
