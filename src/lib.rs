use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn wasm_filter(some_iterable: &JsValue,target: String) -> Result<js_sys::Array, JsValue> {
    let nums = js_sys::Array::new();

    let iterator = js_sys::try_iter(some_iterable)?.ok_or_else(|| {
        "need to pass iterable JS values!"
    })?;

    for x in iterator {
        let x = x?;
        let value  = js_sys::Reflect::get(&x,&JsValue::from_str(&"name")).unwrap_throw();
        let first_name  = js_sys::Reflect::get(&x,&JsValue::from_str(&"firstName")).unwrap_throw();
        let last_name  = js_sys::Reflect::get(&x,&JsValue::from_str(&"lastName")).unwrap_throw();
        let email  = js_sys::Reflect::get(&x,&JsValue::from_str(&"email")).unwrap_throw();

        //let regex = js_sys::RegExp::new(&target,"i");
        //let found = regex.exec(&value);
        
        if value == target {
            nums.push(&x);
        } else if first_name == target {
            nums.push(&x);
        } else if last_name == target {
            nums.push(&x);
        } else if email == target {
            nums.push(&x);
        }

    }

 
    Ok(nums)
}