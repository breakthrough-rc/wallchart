use rscx::{component, html, props};
use serde::Deserialize;

use web_client::server::form::{CellSpan, GridCell, GridLayout, Label, TextInput};

#[derive(Default, Deserialize, Debug, Clone)]
pub struct WorkerProfileFormData {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub street_address: String,
    pub city: String,
    pub region: String,
    pub postal_code: String,
}

#[props]
pub struct WorkerProfileFieldsetProps {
    #[builder(default=WorkerProfileFormData::default())]
    form: WorkerProfileFormData,
}

#[component]
pub fn WorkerProfileFieldset(props: WorkerProfileFieldsetProps) -> String {
    html! {
        <GridLayout class="mt-10">
            <GridCell span=3>
                <Label for_input="last_name">First name</Label>
                <TextInput name="first_name" autocomplete="given-name" value=props.form.first_name />
            </GridCell>

            <GridCell span=3>
                <Label for_input="last_name">Last name</Label>
                <TextInput name="last_name" autocomplete="family-name" value=props.form.last_name />
            </GridCell>

            <GridCell span=4>
                <Label for_input="email">Email address</Label>
                <TextInput input_type="email" name="email" autocomplete="email" value=props.form.email />
            </GridCell>

            <GridCell span=CellSpan::Full>
                <Label for_input="street_address">Street address</Label>
                <TextInput name="street_address" autocomplete="street-address" value=props.form.street_address />
            </GridCell>

            <GridCell span=2 start=1>
                <Label for_input="city">City</Label>
                <TextInput name="city" autocomplete="address-level2" value=props.form.city />
            </GridCell>

            <GridCell span=2>
                <Label for_input="region">State</Label>
                <TextInput name="region" autocomplete="address-level1" value=props.form.region />
            </GridCell>

            <GridCell span=2>
                <Label for_input="postal_code">Zip Code</Label>
                <TextInput name="postal_code" autocomplete="postal-code" value=props.form.postal_code />
            </GridCell>
        </GridLayout>
    }
}
