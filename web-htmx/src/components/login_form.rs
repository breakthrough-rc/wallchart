use rscx::{component, html, props};

#[props]
pub struct LoginFormProps {
    #[builder(default)]
    children: String,
    
    #[builder(setter(into))]
    name: String,
}

#[component]
pub fn LoginForm(props: LoginFormProps) -> String {
    html! {
        <div>
            <h1>Hello {props.name}</h1>
            <div>{props.children}</div>
        </div>
    }
}
