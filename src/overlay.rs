use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct OverlayProps {
    webcam_id: String,
}

#[function_component(Overlay)]
pub fn overlay() -> Html {
    html! {
        <canvas id="overlay-canvas" width="640" height="480"></canvas>
    }
}
