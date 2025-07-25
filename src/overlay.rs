use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use yew::prelude::*;

use crate::controls;

#[function_component(Overlay)]
pub fn overlay() -> Html {
    let store =
        use_context::<UseReducerHandle<controls::ControlState>>().expect("ControlState not found");

    let canvas_ref = use_node_ref();
    let image_ref = use_mut_ref(|| None as Option<HtmlImageElement>);
    let image_loaded = use_state(|| false);

    let img_url = store.image_src.clone();
    let opacity = store.opacity;
    let fit_screen = store.fit_screen;

    // Effect A: Load image only when URL changes
    {
        let image_ref = image_ref.clone();
        let image_loaded = image_loaded.clone();

        use_effect_with(img_url.clone(), move |img_url| {
            image_loaded.set(false);
            if let Some(url) = img_url {
                let img = HtmlImageElement::new().unwrap();
                let image_ref = image_ref.clone();
                let img_for_onload = img.clone();
                let onload_cb = Closure::wrap(Box::new(move || {
                    *image_ref.borrow_mut() = Some(img_for_onload.clone());
                    image_loaded.set(true);
                }) as Box<dyn FnMut()>);

                img.set_onload(Some(onload_cb.as_ref().unchecked_ref()));
                img.set_src(&url);
                onload_cb.forget();
            }
        });
    }

    // Effect B: Redraw canvas when opacity changes or image loads
    {
        let canvas_ref = canvas_ref.clone();
        let image_ref = image_ref.clone();

        use_effect_with((opacity, image_loaded.clone()), move |_| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                // Resize canvas to viewport
                let window = web_sys::window().unwrap();
                canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
                canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

                // Draw image if loaded
                if let Some(img) = &*image_ref.borrow() {
                    context.set_global_alpha(store.opacity);
                    let _ = context.draw_image_with_html_image_element(img, 0.0, 0.0);
                }
            }
        });
    }

    let style = format!(
        "
        position: fixed;
        top: 0; left: 0;
        width: 100vw; height: 100vh;
        object-fit: {};
        z-index: 1;",
        if fit_screen { "contain" } else { "cover" }
    );

    html! {
        <canvas id="overlay-canvas" ref={canvas_ref} style={style} />
    }
}
