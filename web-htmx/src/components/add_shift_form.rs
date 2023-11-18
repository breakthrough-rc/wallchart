use rscx::{component, html, props};
use web_client::server::{
    attrs::Attrs,
    form::{Button, GridCell, GridLayout, Label, TextInput},
};

#[props]
pub struct AddShiftFormProps {
    #[builder(setter(into))]
    action: String,
}

#[component]
pub fn AddShiftForm(props: AddShiftFormProps) -> String {
    html! {
        <div>
            <form hx-post=props.action>
                <div class="pb-12">
                    <p class="mt-1 text-sm leading-6 text-gray-600">
                        Add a new shift
                    </p>
                    <GridLayout class="mt-10">
                        <GridCell span=4>
                            <Label for_input="name">Name</Label>
                            <TextInput name="name" />
                        </GridCell>
                        <GridCell span=4>
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
