use wasm_bindgen::prelude::*;
use gloo::events::EventListener;

pub fn web_sys_htmlmediaelement_play(){
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let body = document.body().expect("document expect to have have a body");
    let button = document.create_element("button")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlButtonElement>()
                    .unwrap();
    button.set_text_content(Some("Play"));

    let paragraph = document.get_element_by_id("message")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlParagraphElement>()
                    .unwrap();
                        

    let on_click = EventListener::new(&button, "click", move |_event| {
        let _video = document.get_element_by_id("my-video")
        .unwrap()
        .dyn_into::<web_sys::HtmlMediaElement>()
        .unwrap();
    
        let promise=_video.play().unwrap();
        
        let _future = async{
            let _result= wasm_bindgen_futures::JsFuture::from(promise).await;
        };
        paragraph.set_text_content(Some("Playing..."));
    
    });

    on_click.forget();     
    body.append_child(&button).unwrap();
}

pub fn web_sys_htmlmediaelement_pause() {
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let body = document.body().expect("document expect to have have a body");
    let button = document.create_element("button")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlButtonElement>()
                    .unwrap();
    button.set_text_content(Some("Pause"));

    let paragraph = document.get_element_by_id("message")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlParagraphElement>()
                    .unwrap();

    let on_click = EventListener::new(&button, "click", move |_event| {
        let _video = document.get_element_by_id("my-video")
        .unwrap()
        .dyn_into::<web_sys::HtmlMediaElement>()
        .unwrap();
    
        _video.pause(); // No need to capture the result
        
        paragraph.set_text_content(Some("Paused..."));
    });

    on_click.forget();     
    body.append_child(&button).unwrap();
}




#[wasm_bindgen(start)]
pub async fn start() {
    web_sys_htmlmediaelement_play();
    web_sys_htmlmediaelement_pause();
  
}