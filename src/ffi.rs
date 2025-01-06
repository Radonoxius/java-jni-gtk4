use jni::JNIEnv;
use jni::objects::{JObject, JString};

use crate::core::{Application, JAPPLICATION};

#[allow(static_mut_refs)]
#[allow(non_snake_case)]
#[no_mangle]
pub(crate) extern "C" fn Java_ix_radon_guiframe_ffi_NativeBridge_getApplication<'local> (
    mut env: JNIEnv<'local>,
    _native_bridge_object: JObject<'local>,
    app: JObject
) {
    let _name = Into::<JString>::into(env.get_field(&app, "name", "Ljava/lang/String;").unwrap().l().unwrap());
    let name = Into::<String>::into(env.get_string(&_name).unwrap());

    unsafe {
        JAPPLICATION = Application {
            name
        };
        JAPPLICATION.start();
    }
}

