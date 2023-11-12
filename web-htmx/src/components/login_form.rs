use rscx::{component, html, props};
use web_client::server::form::{Button, GridCell, GridLayout, Label, TextInput};

#[props]
pub struct LoginFormProps {
    #[builder(setter(into))]
    login_route: String,
}

#[component]
pub fn LoginForm(props: LoginFormProps) -> String {
    html! {
        <div>
            <form hx-post=props.login_route>
                <div class="pb-12">
                    <p class="mt-1 text-sm leading-6 text-gray-600">
                        "pssst: try user@yallchart.com / password"
                    </p>
                    <GridLayout class="mt-10">
                        <GridCell span=4>
                            <Label for_input="email">Email</Label>
                            <TextInput input_type="email" name="email" autocomplete="email" />
                        </GridCell>
                        <GridCell span=4>
                            <Label for_input="password">Password</Label>
                            <TextInput input_type="password" name="password" autocomplete="password" />
                        </GridCell>
                        <GridCell span=4>
                            <div class="mt-6 flex items-center justify-end gap-x-6">
                                <Button kind="submit">Login</Button>
                            </div>
                        </GridCell>
                    </GridLayout>
                </div>
            </form>
        </div>
    }
}
