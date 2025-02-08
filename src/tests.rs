use super::*;
use wasm_bindgen_test::*;
use js_sys::Object;

// 在浏览器中运行测试，默认node
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_basic_string() {
    let message = JsValue::from_str("hello");
    let options = JsValue::from(Object::new());
    let result = md5(&message, options).unwrap();
    assert_eq!(
        result.as_string().unwrap(),
        "5d41402abc4b2a76b9719d911017c592"
    );
}
