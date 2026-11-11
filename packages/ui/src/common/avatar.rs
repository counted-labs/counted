use dioxus::prelude::*;

/// Tinted fill and ink of one hue, light and dark, from each `tailwind.input.css`. The pairs are
/// held at 4.5:1 or better by the test below.
pub(crate) const TINTS: [&str; 5] = [
    "bg-tile-green text-tile-green-content",
    "bg-tile-blue text-tile-blue-content",
    "bg-tile-amber text-tile-amber-content",
    "bg-tile-violet text-tile-violet-content",
    "bg-tile-red text-tile-red-content",
];

pub fn user_color_class(user_id: i32) -> &'static str {
    TINTS[(user_id.unsigned_abs() as usize) % TINTS.len()]
}

pub fn account_color_class(account_id: uuid::Uuid) -> &'static str {
    TINTS[(account_id.as_u128() % TINTS.len() as u128) as usize]
}

/// A friend has no name on this device, only an address: "alex.leroy@x" reads "AL".
pub fn email_initials(email: &str) -> String {
    let local = email.split(['@', '(']).next().unwrap_or_default();
    crate::common::local_storage::initials(&local.replace(['.', '_', '-', '+'], " "))
}

#[derive(PartialEq, Clone, Copy)]
pub enum SizeClass {
    W6,
    W7,
    W8,
    W9,
    W10,
    W12,
}

impl SizeClass {
    pub fn as_class(&self) -> &'static str {
        match self {
            SizeClass::W6 => "w-6 text-xs",
            SizeClass::W7 => "w-7 text-xs",
            SizeClass::W8 => "w-8 text-xs",
            SizeClass::W9 => "w-9 text-sm",
            SizeClass::W10 => "w-10 text-sm",
            SizeClass::W12 => "w-12 text-sm",
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct AvatarProps {
    initials: String,
    /// One of [`TINTS`]; the first when absent.
    color_class: Option<String>,
    #[props(default = SizeClass::W8)]
    size: SizeClass,
    /// Someone with no account behind the name: a dashed outline, no tint.
    #[props(default)]
    guest: bool,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let look = if props.guest {
        "border-[1.5px] border-dashed border-base-300 bg-base-200 text-base-content/70"
    } else {
        props.color_class.as_deref().unwrap_or(TINTS[0])
    };
    let size = props.size.as_class();

    rsx! {
        // Decorative: initials (or the expense emoji) duplicate a name that is always rendered
        // next to it, and read as letter salad otherwise.
        div { class: "avatar avatar-placeholder shrink-0", aria_hidden: "true",
            div { class: "{look} {size} rounded-full ring-2 ring-base-100 font-semibold",
                span { "{props.initials}" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn channel(hex: &str) -> f64 {
        let c = u8::from_str_radix(hex, 16).unwrap() as f64 / 255.0;
        if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
    }

    fn oklch_luminance(l: f64, c: f64, h: f64) -> f64 {
        let (a, b) = (c * h.to_radians().cos(), c * h.to_radians().sin());
        let l_ = (l + 0.396_337_777_4 * a + 0.215_803_757_3 * b).powi(3);
        let m_ = (l - 0.105_561_345_8 * a - 0.063_854_172_8 * b).powi(3);
        let s_ = (l - 0.089_484_177_5 * a - 1.291_485_548 * b).powi(3);
        let r = (4.076_741_662_1 * l_ - 3.307_711_591_3 * m_ + 0.230_969_929_2 * s_).clamp(0.0, 1.0);
        let g = (-1.268_438_004_6 * l_ + 2.609_757_401_1 * m_ - 0.341_319_396_5 * s_).clamp(0.0, 1.0);
        let b = (-0.004_196_086_3 * l_ - 0.703_418_614_7 * m_ + 1.707_614_701 * s_).clamp(0.0, 1.0);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    fn luminance(value: &str) -> f64 {
        if let Some(hex) = value.strip_prefix('#') {
            return 0.2126 * channel(&hex[0..2]) + 0.7152 * channel(&hex[2..4]) + 0.0722 * channel(&hex[4..6]);
        }
        let inner = value.strip_prefix("oklch(").and_then(|v| v.strip_suffix(')')).unwrap_or_else(|| panic!("unparsed colour {value}"));
        let parts: Vec<&str> = inner.split_whitespace().collect();
        let l = parts[0].strip_suffix('%').unwrap().parse::<f64>().unwrap() / 100.0;
        oklch_luminance(l, parts[1].parse().unwrap(), parts[2].parse().unwrap())
    }

    fn contrast(a: f64, b: f64) -> f64 {
        let (hi, lo) = if a > b { (a, b) } else { (b, a) };
        (hi + 0.05) / (lo + 0.05)
    }

    /// Every `--name: value;` of the token, in file order: the light theme's first, the dark one's
    /// after it.
    fn token_values(css: &str, name: &str) -> Vec<String> {
        let needle = format!("{name}:");
        css.lines()
            .filter_map(|line| line.trim().strip_prefix(&needle))
            .map(|v| v.trim().trim_end_matches(';').to_string())
            .collect()
    }

    #[test]
    fn white_on_black_is_twenty_one_to_one() {
        assert!((contrast(luminance("#FFFFFF"), luminance("#000000")) - 21.0).abs() < 0.01);
        assert!((luminance("oklch(100% 0 0)") - 1.0).abs() < 0.001);
    }

    #[test]
    fn every_tint_reads_at_four_and_a_half_to_one_in_both_themes() {
        for shell in ["../mobile/tailwind.input.css", "../web/tailwind.input.css"] {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(shell);
            let css = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
            for tint in TINTS {
                let hue = tint.split_whitespace().next().unwrap().trim_start_matches("bg-");
                let fills = token_values(&css, &format!("--color-{hue}"));
                let inks = token_values(&css, &format!("--color-{hue}-content"));
                assert_eq!((fills.len(), inks.len()), (2, 2), "{shell}: {hue} needs a light and a dark pair");
                for (fill, ink) in fills.iter().zip(&inks) {
                    let ratio = contrast(luminance(fill), luminance(ink));
                    assert!(ratio >= 4.5, "{shell}: {hue} {ink} on {fill} is {ratio:.2}:1");
                }
            }
        }
    }

    #[test]
    fn initials_come_from_the_address_local_part() {
        assert_eq!(email_initials("alex.leroy@proton.me"), "AL");
        assert_eq!(email_initials("theo_moreau@gmail.com"), "TM");
        assert_eq!(email_initials("julie@gmail.com"), "J");
        assert_eq!(email_initials(""), "?");
        assert_eq!(email_initials("Marc (Week-end à Lyon)"), "M");
    }

    #[test]
    fn an_account_keeps_one_tint() {
        let id = uuid::Uuid::from_u128(7);
        assert_eq!(account_color_class(id), account_color_class(id));
        assert_eq!(account_color_class(id), TINTS[2]);
    }

    #[test]
    fn a_user_keeps_one_tint_and_every_tint_is_reached() {
        assert_eq!(user_color_class(42), user_color_class(42));
        assert_eq!(user_color_class(-3), user_color_class(3));
        let reached: std::collections::HashSet<_> = (0..TINTS.len() as i32).map(user_color_class).collect();
        assert_eq!(reached.len(), TINTS.len());
    }
}
