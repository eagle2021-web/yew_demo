use std::fmt::format;
use js_sys::{Reflect, Uint8Array};
use wasm_bindgen::{JsCast, JsValue, prelude::wasm_bindgen};
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, ReadableStreamDefaultReader, Response};
use crate::log_to_console;
use crate::models::ChatCompletionData;
#[wasm_bindgen]
pub struct ReaderUtil { // Renamed to follow UpperCamelCase convention
}

impl ReaderUtil {
    pub async fn fetch_and_log(url: &str) -> Result<String, JsValue> {
        let window = web_sys::window().unwrap();
        let resp = JsFuture::from(window.fetch_with_str(url)).await?.dyn_into::<Response>()?;
        let reader = resp.body().ok_or_else(|| JsValue::from_str("no body"))?.get_reader();
        let reader_js_value = JsValue::from(reader);
        let mut reader = ReadableStreamDefaultReader::from(reader_js_value);
        let mut bytes = Vec::new();
        let mut all = String::new();
        loop {
            let result = JsFuture::from(reader.read()).await?;
            let value = Reflect::get(&result, &JsValue::from_str("value"))?;
            let comp = Reflect::get(&result, &JsValue::from_str("done"));
            if let Ok(js_val) = comp {
                if js_val.as_bool().unwrap() {
                    console::log_1(&"Reading completed".into());
                    break;
                }
            } else {
                console::log_1(&"Error reading stream".into());
            }
            let chunk = Uint8Array::new(&value);
            bytes.extend_from_slice(&chunk.to_vec());
            let mut tmp_bytes = Vec::new();
            tmp_bytes.extend_from_slice(&chunk.to_vec());
            let s = String::from_utf8_lossy(&tmp_bytes);
            let text = String::from_utf8_lossy(&bytes);
            let mut all_parsed = true;
            let mut tmp_all = String::new();
            for each in text.split("data:").into_iter() {
                log_to_console(&format!("{}", each.len()));
                if each.is_empty() || !each.contains("{"){
                    continue;
                }
                let js: serde_json::error::Result<ChatCompletionData> = serde_json::from_str(each);
                if js.is_ok() {
                    let obj = js.unwrap();
                    console::log_1(&format!("obj = {:?}", &obj).into());
                    let content = &obj.choices[0].delta.content;
                    tmp_all.push_str(content);
                } else {
                    all_parsed = false;
                    break;
                }
            }
            if all_parsed {
                all.push_str(&tmp_all);
                bytes.clear();
                continue
            }
            let text = String::from_utf8_lossy(&bytes);
            log_to_console(&text);
        }
        let text = String::from_utf8_lossy(&bytes);
        log_to_console(&text);
        log_to_console(&format!("{}", bytes.len()));
        assert!(bytes.len() <= 1);
        log_to_console(&all);
        Ok(all)
    }
}
#[cfg(test)]
mod tests{
    #[test]
    fn test_strip(){
        let a = "{}";
        let b = a.strip_suffix(" ").unwrap();

    }
}