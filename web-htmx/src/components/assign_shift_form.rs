use rscx::{component, html, props, CollectFragment, CollectFragmentAsync};
use web_client::server::{
    attrs::Attrs,
    form::{Button, GridCell, GridLayout, Label, SelectInput, TextInput},
};
use worksite_service::models::Worker;

#[props]
pub struct AssignShiftFormProps {
    #[builder(setter(into))]
    workers: Vec<Worker>,

    #[builder(setter(into))]
    action: String,
}

#[component]
pub fn AssignShiftForm(props: AssignShiftFormProps) -> String {
    html! {
        <div>
            <form hx-post=props.action>
                <div class="pb-12">
                    <p class="mt-1 text-sm leading-6 text-gray-600">
                        Assign a worker to this shift
                    </p>
                    <GridLayout class="mt-10">
                        <GridCell span=4>
                            <Label for_input="worker_id">Worker</Label>
                            <SelectInput name="worker_id" >
                            {
                                props
                                    .workers
                                    .iter()
                                    .map(|worker| async {
                                        html! {
                                            <option value=worker.id>{worker.full_name()}</option>
                                        }
                                    })
                                    .collect_fragment_async()
                                    .await
                            }
                            </SelectInput>
                        </GridCell>
                        <GridCell span=4>
                            <div class="mt-6 flex items-center justify-end gap-x-6">
                                <Button
                                    onclick="history.go(-1)"
                                    attrs=Attrs::with("data-toggle-action", "close".into())
                                >
                                    Cancel
                                </Button>
                                <Button kind="submit">Assign</Button>
                            </div>
                        </GridCell>
                    </GridLayout>
                </div>
            </form>
        </div>
    }
}
