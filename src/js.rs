use wasm_bindgen::prelude::*;
use super::*;

fn default_clipboard(token :String) -> Clipboard {
    let config = config::ConfigContext {
        config_path: std::path::PathBuf::new(),
        base_url: url::Url::parse(std::str::from_utf8(&base64::decode("aHR0cHM6Ly9hd3MucmVtb3RlLWNsaXBib2FyZC5uZXQ=").unwrap()).unwrap()).unwrap(),
        token,
    };
    return Clipboard::from(config);
}

#[wasm_bindgen]
pub fn open(completion: js_sys::Function) -> () {
    console_error_panic_hook::set_once();
    let clipboard = default_clipboard(String::new());

    use wasm_bindgen::JsValue;

    let completion = move |resp : Result<String, ClipboardError>| {
        let this = JsValue::null();
        match resp {
            Ok(resp) => {
                let resp = JsValue::from(resp);
                completion.call1(&this, &resp).unwrap();
                return ();
            },
            _ => (),
        };
        completion.call1(&this, &JsValue::from_str("error")).unwrap();
    };

    clipboard.open_comp(Box::new(completion)).unwrap();
}

#[wasm_bindgen]
pub fn copy(token:String, input: String) {
    console_error_panic_hook::set_once();
    let clipboard = default_clipboard(token);
    let mut ss = stream::StringStream::from(input);
    let _ = clipboard.copy(&mut ss);
}

#[wasm_bindgen]
pub fn paste(token: String, completion: js_sys::Function) -> () {
    console_error_panic_hook::set_once();
    let clipboard = default_clipboard(token);

    use wasm_bindgen::JsValue;

    let completion = move |resp : Result<String, ClipboardError>| {
        let this = JsValue::null();
        match resp {
            Ok(resp) => {
                let resp = JsValue::from(resp);
                completion.call1(&this, &resp).unwrap();
                return ();
            },
            _ => (),
        };
        completion.call1(&this, &JsValue::from_str("error")).unwrap();
    };

    clipboard.paste_comp(Box::new(completion)).unwrap();
}
