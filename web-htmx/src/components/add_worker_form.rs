use rscx::{component, html, props};
use web_client::server::{
    attrs::Attrs,
    form::{Button, CellSpan, GridCell, GridLayout, Label, Select, TextInput},
};

pub struct AddWorkerFormData {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
}

#[props]
pub struct AddWorkerFormProps {
    #[builder(setter(into))]
    action: String,

    #[builder(default)]
    children: String,

    #[builder(setter(into), default)]
    first_name: String,

    #[builder(setter(into), default)]
    last_name: String,
}

#[component]
pub fn AddWorkerForm(props: AddWorkerFormProps) -> String {
    html! {
        <form hx-post=props.action>
            <div class="pb-12">
                <p class="mt-1 text-sm leading-6 text-gray-600">
                    "Please enter the worker's information."
                </p>
                <GridLayout class="mt-10">
                    <GridCell span=3>
                        <Label for_input="last_name">First name</Label>
                        <TextInput name="first_name" autocomplete="given-name" value=props.first_name />
                    </GridCell>

                    <GridCell span=3>
                        <Label for_input="last_name">Last name</Label>
                        <TextInput name="last_name" autocomplete="family-name" value=props.last_name />
                    </GridCell>

                    <GridCell span=4>
                        <Label for_input="email">Email address</Label>
                        <TextInput input_type="email" name="email" autocomplete="email" />
                    </GridCell>

                    <GridCell span=3>
                        <Label for_input="country">Country</Label>
                        <Select id="country" name="country" autocomplete="country-name">
                            <option default>United States</option>
                            <option>Canada</option>
                            <option>Mexico</option>
                        </Select>
                    </GridCell>

                    <GridCell span=CellSpan::Full>
                        <Label for_input="street_address">Street address</Label>
                        <TextInput name="street_address" autocomplete="street-address" />
                    </GridCell>

                    <GridCell span=2 start=1>
                        <Label for_input="city">City</Label>
                        <TextInput name="city" autocomplete="address-level2" />
                    </GridCell>

                    <GridCell span=2>
                        <Label for_input="region">State</Label>
                        <TextInput name="region" autocomplete="address-level1" />
                    </GridCell>

                    <GridCell span=2>
                        <Label for_input="postal_code">Zip Code</Label>
                        <TextInput name="postal_code" autocomplete="postal-code" />
                    </GridCell>
                    {props.children}
                </GridLayout>
            </div>
            <div class="mt-6 flex items-center justify-end gap-x-6">
                <Button
                    onclick="history.go(-1)"
                    attrs=Attrs::with("data-toggle-action", "close".into())
                >
                    Cancel
                </Button>
                <Button kind="submit">Save</Button>
            </div>
        </form>
    }
}
