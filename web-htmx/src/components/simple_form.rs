use rscx::{component, html, props};
use web_client::server::{
    attrs::Attrs,
    form::{Button, GridCell, GridLayout, Label, TextInput},
};

#[derive(Default, Debug)]
pub struct SimpleFormData {
    pub name: String,
}

#[props]
pub struct SimpleFormProps {
    #[builder(setter(into))]
    action: String,

    #[builder(setter(into), default = "Add a new item".into())]
    description: String,

    #[builder(default)]
    children: String,

    #[builder(default=SimpleFormData::default())]
    data: SimpleFormData,
}

#[component]
pub fn SimpleForm(props: SimpleFormProps) -> String {
    html! {
        <div>
            <form hx-post=props.action>
                <div class="pb-12">
                    <p class="mt-1 text-sm leading-6 text-gray-600">
                        {props.description}
                    </p>
                    <GridLayout class="mt-10">
                        <GridCell span=6>
                            <Label for_input="name">Name</Label>
                            <TextInput name="name" value=&props.data.name />
                        </GridCell>
                        {props.children}
                        <GridCell span=6>
                            <div class="mt-6 flex items-center justify-end gap-x-6">
                                <Button
                                    onclick="history.go(-1)"
                                    attrs=Attrs::with("data-toggle-action", "close".into())
                                >
                                    Cancel
                                </Button>
                                <Button kind="submit">Add</Button>
                            </div>
                        </GridCell>
                    </GridLayout>
                </div>
            </form>
        </div>
    }
}