import { ControlRegistry } from "../registery";
import Transition from "./Transition";

type ToggleState = "opened" | "closed";

type ToggleDelegate = {
  toggleWillOpen?: () => void,
  toggleOpened?: () => void,
  toggleClosed?: () => void,
  shouldToggleCloseOnBodyClick?: boolean,
};

const nullDelegate: ToggleDelegate = {};

// TODO! Support keyboard events
// TODO! Add aria attributes
const Toggle_ = {
  attach(element: HTMLElement, delegate: ToggleDelegate = nullDelegate) {
    let state: ToggleState = "closed";

    console.log("[Toggle::attach()]", element);
    element.dataset.ycControlAttached = "attached";

    const transitionElement = (() => {
      if (element.dataset.ycControl === "transition") {
        return element;
      } else {
        const queriedElement = element.querySelector("[data-yc-control=transition]") as HTMLElement;
        return queriedElement ?? element;
      }
    })();
    const transition = Transition.create(transitionElement);

    const actionElementNodeList = Array.from(element.querySelectorAll("[data-toggle-action]"));
    if (actionElementNodeList.length === 0) {
      console.warn("Toggle control has no action elements with selector: [data-toggle-action].", element);
    }

    actionElementNodeList.forEach((actionElement) => {
      actionElement.addEventListener("click", (event: Event) => {
        event.stopPropagation();

        const actionKey = (event.target as HTMLElement).dataset.toggleAction;

        if (actionKey && actionKey in actions) {
          actions[actionKey]();
        } else {
          handleToggleStateChange();
        }
      });
    });

    const shouldCloseOnBodyClick = delegate.shouldToggleCloseOnBodyClick ?? true;

    const actions: Record<string, Function> = {
      async open() {
        state = "opened";
        delegate.toggleWillOpen?.();

        await transition.enter();

        if (shouldCloseOnBodyClick) {
          document.body.addEventListener("click", handleBodyClick);
        }
        delegate.toggleOpened?.();
      },

      async close() {
        state = "closed";
        await transition.leave();

        if (shouldCloseOnBodyClick) {
          document.body.removeEventListener("click", handleBodyClick);
        }
        delegate.toggleClosed?.();
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

    return actions;
  },
};

function init(registry: ControlRegistry) {
  registry.registerControl("toggle", Toggle_);
}

export default {
  init,
  ...Toggle_,
}
