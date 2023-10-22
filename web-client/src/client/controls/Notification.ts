import events from "./events";
import { ControlRegistry } from "./registery";
import Transition from "./Transition";

type ErrorNotificationRequest = {
  kind: 'ERROR',
  message: string,
};

type GenericNotificationRequest = {
  kind: 'GENERIC',
};

type NotificationRequest = ErrorNotificationRequest | GenericNotificationRequest;

const Notifications = {
  get region(): HTMLElement {
    const region = document.getElementById('notification-live-region');
    if (!region) throw new Error("Could not find notification live region.");
    return region;
  },

  get content(): HTMLElement {
    const content = Notifications.region.querySelector(":scope > section") as HTMLElement;
    if (!content) throw new Error("Could not find notification live region main content.");
    return content;
  },

  init() {
    events.on("yc:notificationRequest", (request: NotificationRequest) => {
      switch (request.kind) {
        case "ERROR":
          Notifications.showError(request);
          break;
        case "GENERIC":
          console.warn("Generic notification request not implemented yet.");
          break;
      }
    });
  },

  async appendNotification(notification: HTMLElement) {
    const button = notification.querySelector("button[data-notification-close]");
    if (!button) throw new Error("Could not find notification close button.");

    // Fragments don't provide a reference to the node appended to the DOM
    // Cloning the childnodes array and pulling the first item to obtain a reference.
    const [notificationElement] = [...notification.children];
    const transition = Transition.create(notificationElement);

    const removeNotification = async () => {
      await transition.leave();
      Notifications.content.removeChild(notificationElement);
      button.removeEventListener("click", removeNotification);
    };

    button.addEventListener("click", removeNotification);

    Notifications.content.appendChild(notification);
    await transition.enter();
  },

  async showError(request: ErrorNotificationRequest) {
    const tpl = document.getElementById("tpl-error-notification") as HTMLTemplateElement;
    if (!tpl) throw new Error("Can not show ErrorNotification. Element with id `tpl-error-notification` not found.");

    const notification = tpl.content.cloneNode(true) as HTMLElement;
    const messageElement = notification.querySelector("[data-error-message]");
    if (!messageElement) throw new Error("Could not find element with attribute `data-error-message` in template.");

    messageElement.textContent = request.message || "Unknown error";
    return Notifications.appendNotification(notification);
  },
};

function init(registry: ControlRegistry) {
  Notifications.init();

  registry.registerGlobalApi({
    showErrorNotification(message: string) {
      Notifications.showError({
        kind: "ERROR",
        message,
      });
    },
  });
}

export default {
  init,
}
