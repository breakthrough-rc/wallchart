use super::transition::Transition;
use super::yc_control::YcControl;
use crate::server::attrs::Attrs;
use rscx::{component, html, props};

pub enum ModalSize {
    Small,
    Medium,
    Large,
    SmallScreen,
    MediumScreen,
    Custom(String),
}

#[props]
pub struct ModalProps {
    #[builder(default = ModalSize::Medium)]
    size: ModalSize,

    #[builder(setter(into))]
    children: String,
}

#[component]
pub fn Modal(props: ModalProps) -> String {
    html! {
      <YcControl
        control="modal"
        class="relative z-10"
        role="dialog"
        aria_labelledby="modal-title"
        attrs=Attrs::with("aria-modal", "true".into())
      >
        <Transition
          class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"
          enter="ease-out duration-300"
          enter_from="opacity-0"
          enter_to="opacity-100"
          leave="ease-in duration-200"
          leave_from="opacity-100"
          leave_to="opacity-0"
        />
        <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
          <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
            <Transition
              class={
                let m_width = match props.size {
                  ModalSize::Small => "sm:max-w-sm".to_string(),
                  ModalSize::Medium => "sm:max-w-md".to_string(),
                  ModalSize::Large => "sm:max-w-lg".to_string(),
                  ModalSize::SmallScreen => "sm:max-w-screen-sm".to_string(),
                  ModalSize::MediumScreen => "sm:max-w-screen-md".to_string(),
                  ModalSize::Custom(width) => width,
                };
                format!("relative transform overflow-hidden rounded-lg bg-white px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 w-full {} sm:p-6", m_width)
              }
              attrs=Attrs::with("data-modal-panel", "true".into())
              enter="ease-out duration-300"
              enter_from="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
              enter_to="opacity-100 translate-y-0 sm:scale-100"
              leave="ease-in duration-200"
              leave_from="opacity-100 translate-y-0 sm:scale-100"
              leave_to="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
            >
              <div>
                {props.children}
              </div>
            </Transition>
          </div>
        </div>
      </YcControl>
    }
}
