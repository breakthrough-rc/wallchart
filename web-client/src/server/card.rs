use rscx::{component, html, props};

#[props]
pub struct CardProps {
    children: String,

    #[builder(default = false)]
    padded: bool,
}

#[component]
pub fn Card(props: CardProps) -> String {
    html! {
        <div class="overflow-hidden shadow ring-1 ring-black ring-opacity-5 sm:rounded-lg">
            {match props.padded {
                true => html! {
                    <div class="px-4 py-5 sm:p-6">
                        {props.children}
                    </div>
                },
                false => props.children,
            }}
        </div>
    }
}
