use rscx::{component, html, props};
use web_client::server::form::{Button, GridCell, GridLayout, Label, TextInput};

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
                        Add a new form
                    </p>
                    <GridLayout class="mt-10">
                        <GridCell span=4>
                            <Label for_input="name">Name</Label>
                            <TextInput input_type="name" name="name" />
                        </GridCell>
                        <GridCell span=4>
                            <div class="mt-6 flex items-center justify-end gap-x-6">
                                <Button kind="submit">Add</Button>
                            </div>
                        </GridCell>
                    </GridLayout>
                </div>
            </form>
        </div>
    }
}
