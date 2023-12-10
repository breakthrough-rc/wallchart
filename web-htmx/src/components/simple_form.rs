use rscx::{component, html, props};
use serde::Deserialize;
use web_client::server::{
    attrs::Attrs,
    form::{Button, GridCell, GridLayout, Label, TextInput},
};

#[derive(Default, Deserialize, Debug)]
pub struct SimpleFormData {
    pub name: String,
}

#[props]
pub struct SimpleFormProps {
    #[builder(setter(into))]
    action: String,

    #[builder(setter(into), default)]
    description: String,

    #[builder(setter(into), default = "Name".into())]
    label_text: String,

    #[builder(setter(into), default = "name".into())]
    input_name: String,

    #[builder(default)]
    children: String,

    #[builder(default=SimpleFormData::default())]
    data: SimpleFormData,

    #[builder(setter(into), default="Add".into())]
    submit_button_text: String,
}

#[component]
pub fn SimpleForm(props: SimpleFormProps) -> String {
    html! {
        <div>
            <form hx-post=props.action>
                <div class="pb-12">
                    {
                        match !props.description.is_empty() {
                            true => html! {
                                <p class="mt-1 text-sm leading-6 text-gray-600">
                                    {&props.description}
                                </p>
                            },
                            false => "".into(),
                        }
                    }
                    <GridLayout class="mt-10">
                        <GridCell>
                            <Label for_input=&props.input_name>{&props.label_text}</Label>
                            <TextInput name=&props.input_name value=&props.data.name />
                        </GridCell>
                        {props.children}
                        <GridCell>
                            <div class="mt-6 flex items-center justify-end gap-x-6">
                                <Button
                                    onclick="history.go(-1)"
                                    attrs=Attrs::with("data-toggle-action", "close".into())
                                >
                                    Cancel
                                </Button>
                                <Button kind="submit">{props.submit_button_text}</Button>
                            </div>
                        </GridCell>
                    </GridLayout>
                </div>
            </form>
        </div>
    }
}
