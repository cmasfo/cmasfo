
use wasm_bindgen::prelude::*;
use wasm_bindgen::{
  JsCast,
  JsValue,
};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
  Request,
  RequestInit,
  RequestMode,
  Response,
};

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
  
  let window = web_sys::window().unwrap();
  let document = window.document().unwrap();
  let body = document.body().unwrap();

  // create paragraph
  let p = document.create_element("p")?;
  p.set_id("time");
  p.set_inner_html("loading time...");
  body.append_child(&p)?;

  // create button
  let button = document.create_element("button")?;
  button.set_inner_html("Update time");
  body.append_child(&button)?;

  // add event listener to button (update_time)
  // closure should be async
  let closure = Closure::wrap(Box::new(move || {
    wasm_bindgen_futures::spawn_local(update_time());
  }) as Box<dyn FnMut()>);

  button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
  closure.forget();

  update_time().await;

  Ok(())

}

#[wasm_bindgen]
pub async fn update_time() {
  
  // console
  web_sys::console::log_1(&JsValue::from_str("update_time is called"));

  let mut opts = RequestInit::new();
  opts.method("GET");
  opts.mode(RequestMode::Cors);

  let url = format!("/time");

  let request = Request::new_with_str_and_init(&url, &opts).unwrap();

  let window = web_sys::window().unwrap();
  let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();

  assert!(resp_value.is_instance_of::<Response>());
  let resp: Response = resp_value.dyn_into().unwrap();

  let text = JsFuture::from(resp.text().unwrap()).await.unwrap();

  let now = text.as_string().unwrap();
  let window = web_sys::window().unwrap();
  let document = window.document().unwrap();
  let p = document.get_element_by_id("time").unwrap();
  p.set_inner_html(&now);

}
