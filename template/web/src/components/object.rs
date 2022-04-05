use {{crate_name}};
use uikit_rs as uk;
use yew::{classes, function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct ObjectProps {
    #[prop_or_default]
    pub object: Option<{{crate_name}}::Object>,
}

#[function_component(Object)]
pub fn object(ObjectProps { object }: &ObjectProps) -> Html {
    html! {
        <span class={classes!(uk::Text::Meta)}>
        {
            match object {
                Some(obj) => format!("object: {}", obj.name ),
                None => "No object".to_string()
            }
        }
        </span>
    }
}
