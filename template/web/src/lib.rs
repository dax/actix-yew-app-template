use components::object;
use ::{{crate_name}}::Object;
use reqwasm::http::Request;
use uikit_rs as uk;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;

#[wasm_bindgen(module = "/js/api.js")]
extern "C" {
    fn get_api_base_url() -> String;
}

#[function_component(App)]
pub fn app() -> Html {
    let object = use_state(|| None);
    {
        let object = object.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_object: Object =
                        Request::get(&format!("{}/hello", get_api_base_url()))
                            .send()
                            .await
                            .unwrap() // TODO
                            .json()
                            .await
                            .unwrap(); // TODO
                    object.set(Some(fetched_object));
                });
                || ()
            },
            (),
        );
    }

    html! {
        <uk::Section style={uk::SectionStyle::Default}>
          <uk::Container size={uk::ContainerSize::Small}>
            <object::Object object={(*object).clone()} />
          </uk::Container>
        </uk::Section>
    }
}
