//! The device's own calendar: its zone offset at a UTC instant, as chrono's `Local` computed it on
//! each target (docs/plans/chrono-to-jiff.md §3.2, D5).

use shared::time::{Date, DateTime};

/// Seconds east of UTC on this device at the naive UTC date-time `utc`. The browser's own
/// `getTimezoneOffset`, as chrono's `wasmbind` read it.
#[cfg(target_arch = "wasm32")]
pub fn offset_at(utc: DateTime) -> i32 {
    let at = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(utc.unix_millis() as f64));
    -(at.get_timezone_offset() as i32) * 60
}

#[cfg(not(target_arch = "wasm32"))]
pub fn offset_at(utc: DateTime) -> i32 {
    let Ok(at) = jiff::Timestamp::from_second(utc.unix_secs()) else { return 0 };
    system_zone().to_offset(at).seconds()
}

/// chrono's `Local`: `TZ` when it is set; otherwise the zone the platform names, read on every call
/// (Android's `persist.sys.timezone`, iOS's CoreFoundation, Linux's `/etc/localtime` link), with its
/// rules from the freshest database; then jiff's own detection; UTC when nothing answers.
#[cfg(not(target_arch = "wasm32"))]
fn system_zone() -> jiff::tz::TimeZone {
    use jiff::tz::TimeZone;
    if std::env::var_os("TZ").is_some() {
        if let Ok(zone) = TimeZone::try_system() {
            return zone;
        }
    }
    if let Ok(name) = iana_time_zone::get_timezone() {
        if let Ok(zone) = database().get(&name) {
            return zone;
        }
        if let Some(zone) = zoneinfo_file(&name) {
            return zone;
        }
    }
    TimeZone::try_system().unwrap_or(TimeZone::UTC)
}

/// chrono's last resort, kept for a sandbox (iOS) that lets the app read a zone file it cannot list.
#[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
fn zoneinfo_file(name: &str) -> Option<jiff::tz::TimeZone> {
    let data = std::fs::read(format!("/usr/share/zoneinfo/{name}")).ok()?;
    jiff::tz::TimeZone::tzif(name, &data).ok()
}

#[cfg(target_os = "android")]
fn zoneinfo_file(_name: &str) -> Option<jiff::tz::TimeZone> {
    None
}

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
fn database() -> &'static jiff::tz::TimeZoneDatabase {
    jiff::tz::db()
}

/// jiff 0.2.39 reads only the ROM's copy (`/system`), which no update reaches; the APEX module
/// Play system updates refresh is where chrono looked first.
#[cfg(target_os = "android")]
fn database() -> &'static jiff::tz::TimeZoneDatabase {
    static DATABASE: std::sync::OnceLock<jiff::tz::TimeZoneDatabase> = std::sync::OnceLock::new();
    DATABASE.get_or_init(|| android_database(|name| std::env::var(name).ok()).unwrap_or_else(|| jiff::tz::db().clone()))
}

/// chrono's `open_android_tz_data_file` order: the APEX module, the data partition, the ROM.
#[cfg(any(target_os = "android", test))]
fn android_tzdata_paths(env: impl Fn(&str) -> Option<String>) -> Vec<String> {
    [("ANDROID_TZDATA_ROOT", "/etc/tz"), ("ANDROID_DATA", "/misc/zoneinfo/current"), ("ANDROID_ROOT", "/usr/share/zoneinfo")]
        .into_iter()
        .filter_map(|(name, dir)| env(name).map(|root| format!("{root}{dir}/tzdata")))
        .collect()
}

#[cfg(any(target_os = "android", test))]
fn android_database(env: impl Fn(&str) -> Option<String>) -> Option<jiff::tz::TimeZoneDatabase> {
    android_tzdata_paths(env)
        .into_iter()
        .filter(|path| std::fs::File::open(path).is_ok())
        .find_map(|path| jiff::tz::TimeZoneDatabase::from_concatenated_path(path).ok())
}

pub fn to_local(utc: DateTime) -> DateTime {
    utc.checked_add_seconds(offset_at(utc) as i64).unwrap_or(utc)
}

pub fn today() -> Date {
    to_local(DateTime::now_utc()).date()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn android_rules_come_from_the_apex_module_first() {
        let env = |name: &str| match name {
            "ANDROID_TZDATA_ROOT" => Some("/apex/com.android.tzdata".to_string()),
            "ANDROID_DATA" => Some("/data".to_string()),
            "ANDROID_ROOT" => Some("/system".to_string()),
            _ => None,
        };
        assert_eq!(
            android_tzdata_paths(env),
            [
                "/apex/com.android.tzdata/etc/tz/tzdata",
                "/data/misc/zoneinfo/current/tzdata",
                "/system/usr/share/zoneinfo/tzdata",
            ]
        );
    }

    /// The finding it fixes, measured on two real Android databases: `TZPROOF_ROM` the ROM's copy
    /// (tzdata2018e, Android 9) and `TZPROOF_APEX` an updated module (tzdata2025a). Brazil dropped
    /// daylight saving in 2019, so only the updated rules say -03 in November.
    #[test]
    #[ignore]
    fn an_updated_module_wins_over_the_roms_copy() {
        let (rom, apex) = (std::env::var("TZPROOF_ROM").unwrap(), std::env::var("TZPROOF_APEX").unwrap());
        let root = std::env::temp_dir().join("counted-tzproof");
        for (dir, source) in [("apex/etc/tz", &apex), ("system/usr/share/zoneinfo", &rom)] {
            std::fs::create_dir_all(root.join(dir)).unwrap();
            std::fs::copy(source, root.join(dir).join("tzdata")).unwrap();
        }
        let at: jiff::Timestamp = "2025-11-30T12:00:00Z".parse().unwrap();
        let offset = |db: &jiff::tz::TimeZoneDatabase| db.get("America/Sao_Paulo").unwrap().to_offset(at).seconds() / 3600;
        let rom_only = jiff::tz::TimeZoneDatabase::from_concatenated_path(root.join("system/usr/share/zoneinfo/tzdata")).unwrap();
        assert_eq!(offset(&rom_only), -2, "the ROM's 2018 rules still had daylight saving");
        let root_text = root.to_string_lossy().replace('\\', "/");
        let env = |name: &str| match name {
            "ANDROID_TZDATA_ROOT" => Some(format!("{root_text}/apex")),
            "ANDROID_DATA" => Some(format!("{root_text}/data")),
            "ANDROID_ROOT" => Some(format!("{root_text}/system")),
            _ => None,
        };
        assert_eq!(offset(&android_database(env).unwrap()), -3);
    }
}
