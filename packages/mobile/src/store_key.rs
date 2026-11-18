//! The store key from the platform (docs/plans/security-3.3.0.md §4): Android unwraps it with a
//! Keystore key in `MainActivity.storeKey`, iOS keeps it as a Keychain item of this device only.
//! Installed before the first store read, which happens before launch.

use ui::common::store_seal::{KeyRequest, StoreKey};

pub fn install() {
    #[cfg(target_os = "android")]
    let provider = android::provider;
    #[cfg(target_os = "ios")]
    let provider = ios::provider;
    let state = ui::common::store_seal::install(std::sync::Arc::new(provider));
    tracing::info!("counted: store key {}", describe(state));
}

fn describe(state: StoreKey) -> &'static str {
    match state {
        StoreKey::Plain => "not available, store plain",
        StoreKey::Ready { created: true, .. } => "created",
        StoreKey::Ready { created: false, .. } => "ready",
        StoreKey::Locked => "locked",
        StoreKey::Lost => "lost",
    }
}

#[cfg(target_os = "android")]
mod android {
    use super::{KeyRequest, StoreKey};
    use jni::objects::{JByteArray, JObject, JValue};

    const READY: u8 = 0;
    const CREATED: u8 = 1;
    const LOST: u8 = 3;

    /// `MainActivity` answers a status byte, followed by the key when there is one. A build whose
    /// activity has no such method (a `dx serve` build) stays plain; any other failure is locked,
    /// never plain, so a broken Keystore cannot turn sealing off.
    pub fn provider(request: KeyRequest) -> StoreKey {
        match call(&request) {
            Ok(Some(answer)) => decode(&answer),
            Ok(None) => StoreKey::Plain,
            Err(e) => {
                tracing::error!("counted: store key call failed: {e}");
                StoreKey::Locked
            }
        }
    }

    fn decode(answer: &[u8]) -> StoreKey {
        let key = || answer.get(1..33).and_then(|k| <[u8; 32]>::try_from(k).ok());
        match (answer.first().copied(), key()) {
            (Some(READY), Some(key)) => StoreKey::Ready { key, created: false },
            (Some(CREATED), Some(key)) => StoreKey::Ready { key, created: true },
            (Some(LOST), _) => StoreKey::Lost,
            _ => StoreKey::Locked,
        }
    }

    fn call(request: &KeyRequest) -> jni::errors::Result<Option<Vec<u8>>> {
        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
        let mut env = vm.attach_current_thread()?;
        let activity = unsafe { JObject::from_raw(ctx.context().cast()) };
        let result = match request {
            KeyRequest::Open { may_create } => {
                env.call_method(&activity, "storeKey", "(Z)[B", &[JValue::Bool(u8::from(*may_create))])
            }
            KeyRequest::Reset => env.call_method(&activity, "resetStoreKey", "()[B", &[]),
        };
        let value = match result {
            Ok(value) => value,
            Err(jni::errors::Error::JavaException) => {
                env.exception_clear()?;
                return Ok(None);
            }
            Err(e) => return Err(e),
        };
        let array = JByteArray::from(value.l()?);
        Ok(Some(env.convert_byte_array(&array)?))
    }
}

#[cfg(target_os = "ios")]
mod ios {
    use super::{KeyRequest, StoreKey};
    use core_foundation::base::{CFType, CFTypeRef, TCFType};
    use core_foundation::boolean::CFBoolean;
    use core_foundation::data::{CFData, CFDataRef};
    use core_foundation::dictionary::CFDictionary;
    use core_foundation::string::{CFString, CFStringRef};
    use security_framework_sys::access_control::kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly;
    use security_framework_sys::base::{errSecItemNotFound, errSecSuccess};
    use security_framework_sys::item::{
        kSecAttrAccount, kSecAttrService, kSecAttrSynchronizable, kSecClass, kSecClassGenericPassword, kSecReturnData,
        kSecValueData,
    };
    use security_framework_sys::keychain_item::{SecItemAdd, SecItemCopyMatching, SecItemDelete};

    extern "C" {
        static kSecAttrAccessible: CFStringRef;
    }

    const SERVICE: &str = "fr.counted.app.store";
    const ACCOUNT: &str = "store-key-v1";

    fn constant(key: CFStringRef) -> CFString {
        unsafe { CFString::wrap_under_get_rule(key) }
    }

    /// Generic password of this service and account, never synchronised to iCloud Keychain.
    fn item(extra: &[(CFString, CFType)]) -> CFDictionary<CFString, CFType> {
        let mut pairs = unsafe {
            vec![
                (constant(kSecClass), constant(kSecClassGenericPassword).as_CFType()),
                (constant(kSecAttrService), CFString::new(SERVICE).as_CFType()),
                (constant(kSecAttrAccount), CFString::new(ACCOUNT).as_CFType()),
                (constant(kSecAttrSynchronizable), CFBoolean::false_value().as_CFType()),
            ]
        };
        pairs.extend_from_slice(extra);
        CFDictionary::from_CFType_pairs(&pairs)
    }

    fn read() -> Result<Vec<u8>, i32> {
        let query = item(&[(unsafe { constant(kSecReturnData) }, CFBoolean::true_value().as_CFType())]);
        let mut out: CFTypeRef = std::ptr::null();
        let status = unsafe { SecItemCopyMatching(query.as_concrete_TypeRef(), &mut out) };
        if status != errSecSuccess {
            return Err(status);
        }
        Ok(unsafe { CFData::wrap_under_create_rule(out as CFDataRef) }.bytes().to_vec())
    }

    /// `SecItemAdd` alone: a duplicate is an error, never an update, so an existing key is never
    /// overwritten (the `security-framework` password helpers update on a duplicate).
    fn add(key: &[u8; 32]) -> i32 {
        let attributes = item(&unsafe {
            [
                (constant(kSecValueData), CFData::from_buffer(key).as_CFType()),
                (constant(kSecAttrAccessible), constant(kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly).as_CFType()),
            ]
        });
        unsafe { SecItemAdd(attributes.as_concrete_TypeRef(), std::ptr::null_mut()) }
    }

    pub fn provider(request: KeyRequest) -> StoreKey {
        match request {
            KeyRequest::Reset => {
                unsafe { SecItemDelete(item(&[]).as_concrete_TypeRef()) };
                StoreKey::Plain
            }
            KeyRequest::Open { may_create } => match read() {
                Ok(bytes) => <[u8; 32]>::try_from(bytes.as_slice()).map_or(StoreKey::Lost, |key| StoreKey::Ready { key, created: false }),
                Err(status) if status == errSecItemNotFound && may_create => {
                    let key = ui::crypto::generate_key();
                    match (add(&key), read()) {
                        (status, Ok(back)) if status == errSecSuccess && back == key => StoreKey::Ready { key, created: true },
                        _ => StoreKey::Locked,
                    }
                }
                Err(status) if status == errSecItemNotFound => StoreKey::Lost,
                Err(status) => {
                    tracing::error!("counted: store key keychain read failed: {status}");
                    StoreKey::Locked
                }
            },
        }
    }
}
