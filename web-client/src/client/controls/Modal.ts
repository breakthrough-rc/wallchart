import { ControlRegistry } from "../registery";
import Toggle from "./Toggle";
import query from "../query"

const Modal = {
  async attach(element: HTMLElement) {
    console.log("Modal::attach()", element);
    const toggle = Toggle.attach(element);

    const modalPanelElement = query(element, "[data-modal-panel]") as HTMLElement;
    const togglePanel = Toggle.attach(modalPanelElement, {
      async toggleClosed() {
        await toggle.close();
        element.parentElement?.removeChild(element);
      }
    });

    await toggle.open();
    await togglePanel.open();
  }
}

function init(registry: ControlRegistry) {
  registry.registerControl("modal", Modal);
}

export default {
  init,
}