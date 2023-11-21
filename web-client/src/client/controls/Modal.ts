import { ControlRegistry } from "../registery";
import Toggle from "./Toggle";
import query from "../query"

type ConfirmDeleteCommand = {
  title?: string,
  message?: string,
  deleteHref: string,
};

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
  },

  confirmDelete({
    title,
    message,
    deleteHref,
  }: ConfirmDeleteCommand) {
    const tpl = query(document, "#tpl-confirm-delete-modal") as HTMLTemplateElement;
    const modalElement = tpl.content.cloneNode(true) as HTMLElement;

    const Elements = query.all(modalElement, {
      title: "[data-confirm-delete-title]",
      message: "[data-confirm-delete-message]",
    });

    if (title) Elements.title.textContent = title;
    if (message) Elements.message.textContent = message;

    const deleteActionElement = query(modalElement, "[data-confirm-action=\"delete\"]");
    deleteActionElement.addEventListener("click", () => {
      // TODO REMOVE hard dependency on htmx
      (window as any).htmx.ajax("DELETE", deleteHref, { target: document.body, swap: "beforeend" });
    }, { once: true });

    const modalContentElement = query(document, "#modal-live-region [data-modal-content]");
    modalContentElement.appendChild(modalElement);
  }
};

function init(registry: ControlRegistry) {
  registry.registerControl("modal", Modal);

  registry.registerGlobalApi({
    confirmDelete: Modal.confirmDelete,
  });
}

export default {
  init,
}