use yew::prelude::*;

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlVideoElement, MediaStreamConstraints};

use crate::controls;

#[function_component(Webcam)]
pub fn webcam() -> Html {
    let controls =
        use_context::<UseReducerHandle<controls::ControlState>>().expect("ControlCtx not found");
    let video_ref = use_node_ref();

    {
        let video_ref = video_ref.clone();
        use_effect_with((), move |_| {
            let constraints = MediaStreamConstraints::new();
            constraints.set_video(&JsValue::TRUE);

            //Get the camera
            let media_devices = web_sys::window()
                .unwrap()
                .navigator()
                .media_devices()
                .unwrap();
            let promise = media_devices
                .get_user_media_with_constraints(&constraints)
                .unwrap();

            spawn_local(async move {
                let stream = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
                let media_stream: web_sys::MediaStream = stream.dyn_into().unwrap();

                if let Some(video) = video_ref.cast::<HtmlVideoElement>() {
                    video.set_src_object(Some(&media_stream));
                    let _ = video.play();
                }
            });
        });
    }

    let style = format!(
        "
        position: fixed;
        top: 0; left: 0;
        width: 100vw; height: 100vh;
        object-fit: {};
        z-index: -1;",
        if controls.fit_screen {
            "contain"
        } else {
            "cover"
        }
    );
    html! {
        <>
        <video ref={video_ref} autoplay=true style={style}></video>
        </>
    }
}
