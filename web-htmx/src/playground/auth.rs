#![allow(unused_braces)]

use rscx::{component, html, props};

use web_client::server::button::PrimaryButton;

#[component]
pub fn AuthPlayground() -> String {
    html! {
      <section class="py-8">
          <h2 class="text-xl font-bold">Auth Testing</h2>
          <div class="flex gap-2">
              <PrimaryButton
                  tag="a"
                  href="/wallchart"
              >
                  Authenticated page link
              </PrimaryButton>
          </div>
      </section>
    }
}
