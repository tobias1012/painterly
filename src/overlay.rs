use yew::prelude::*;

use crate::controls;

#[derive(Properties, PartialEq)]
struct OverlayProps {
    webcam_id: String,
}

#[function_component(Overlay)]
pub fn overlay() -> Html {
    let store =
        use_context::<UseReducerHandle<controls::ControlState>>().expect("ControlState not found");
    let canvas_ref = use_node_ref();
    //Setup WebGL for canvas

    {
        let canvas_ref = canvas_ref.clone();
        use_effect_with((), move |_| {
            if let Some(canvas) = canvas_ref.cast::<web_sys::HtmlCanvasElement>() {
                // Initialize WebGL context
                let _gl = canvas
                    .get_context("webgl")
                    .expect("Failed to get WebGL context")
                    .unwrap();
            }
            || ()
        });
    }

    let style = format!(
        "
        position: fixed;
        top: 0; left: 0;
        width: 100vw; height: 100vh;
        object-fit: {};
        z-index: 1;",
        if store.fit_screen { "contain" } else { "cover" }
    );

    html! {
        <canvas id="overlay-canvas" ref={canvas_ref} style={style} ></canvas>
    }
}
