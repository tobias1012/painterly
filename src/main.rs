use yew::prelude::*;

mod webcam;
use webcam::Webcam;

mod overlay;

mod controls;
use controls::{ControlState, Controls};

#[function_component(App)]
fn app() -> Html {
    let store = use_reducer(ControlState::new);

    html! {
        <div>
            <ContextProvider<UseReducerHandle<ControlState>> context={store}>
                <Controls />
                <Webcam />
            </ContextProvider<UseReducerHandle<ControlState>>>
        </div>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
