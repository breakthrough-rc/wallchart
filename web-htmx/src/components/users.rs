use auth_service::models::User;
use rscx::{component, html, props, CollectFragmentAsync};

#[props]
pub struct UsersProps {
    users: Vec<User>,
}

#[component]
pub fn Users(props: UsersProps) -> String {
    html! {
        <table class="min-w-full divide-y divide-gray-300">
            <thead class="bg-gray-50">

                <tr>
                    <th scope="col" class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-3">Email</th>
                    <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">Role</th>
                </tr>
            </thead>
            <tbody class="bg-white">
                {
                    props
                        .users
                        .iter()
                        .map(|user| async {
                            html! {
                                <User
                                    user=user.clone()
                                />
                            }
                        })
                        .collect_fragment_async()
                        .await
                }
            </tbody>
        </table>
    }
}

#[props]
pub struct UserProps {
    user: User,
}

#[component]
pub fn User(props: UserProps) -> String {
    html! {
        <tr class="border-t border-gray-300" data-loading-states>
            <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">
                {props.user.email}
            </td>
            <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">Organizer</td>
        </tr>
    }
}
