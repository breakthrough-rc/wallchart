import { ControlRegistry } from "../registery";
import Transition from "./Transition";

type ToggleState = "opened" | "closed";

// TODO! Support keyboard events
// TODO! Add aria attributes
const Toggle_ = {
  attach(element: HTMLElement) {
    let state: ToggleState = "closed";

    console.log("[Toggle::attach()]", element);
    element.dataset.ycControlAttached = "attached";

    const transitionElement = element.querySelector("[data-yc-control=transition") as HTMLElement;
    const transition = Transition.create(transitionElement);

    element.querySelector("[data-toggle-action]")?.addEventListener("click", (event: Event) => {
      event.stopPropagation();

      const actionKey = (event.target as HTMLElement).dataset.toggleAction;

      if (actionKey && actionKey in actions) {
        actions[actionKey]();
      } else {
        handleToggleStateChange();
      }
    });

    const actions: Record<string, Function> = {
      open() {
        state = "opened";
        transition.enter();

        document.body.addEventListener("click", handleBodyClick);
      },

      close() {
        state = "closed";
        transition.leave();

        document.body.removeEventListener("click", handleBodyClick);
      },
    };

    const handleBodyClick = (event: Event) => {
      if (event.target === element || element.contains(event.target as Node)) {
        event.preventDefault();
        return false;
      }

      actions.close();
    };

    const handleToggleStateChange = () => {
      switch (state) {
        case "opened":
          actions.close();
          break;

        case "closed":
          actions.open();
          break;
      }
    };
  },
};

function init(registry: ControlRegistry) {
  registry.registerControl("toggle", Toggle_);
}

export default {
  init,
  ...Toggle_,
}
