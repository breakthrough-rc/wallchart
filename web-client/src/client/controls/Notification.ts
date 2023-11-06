import events from "../events";
import { ControlRegistry } from "../registery";
import Toggle from "./Toggle";

type NotificationRequest = {
  kind: 'SUCCESS' | 'ERROR' | 'GENERIC',
  message: string,
};

let showFromTemplate = (tplSelector: string) => async (request: NotificationRequest) => {
  const tpl = document.querySelector(tplSelector) as HTMLTemplateElement;
  if (!tpl) throw new Error(`Can not show Notification. Element selector "${tplSelector}" not found.`);

  const notification = tpl.content.cloneNode(true) as HTMLElement;
  const messageElement = notification.querySelector("[data-notification-message]");
  if (!messageElement) throw new Error("Could not find element with attribute `data-notification-message` in template.");

  messageElement.textContent = request.message || "Everything is all good!";
  return Notifications.appendNotification(notification);
};

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
        case "SUCCESS":
          Notifications.showSuccess(request);
          break;
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
    // Fragments don't provide a reference to DOM element, first child is actual element attached.
    const notificationElement = notification.firstElementChild!;

    const toggle = Toggle.attach(notificationElement as HTMLElement, {
      toggleWillOpen: () => Notifications.content.appendChild(notification),
      toggleClosed: () => Notifications.content.removeChild(notificationElement),
      shouldToggleCloseOnBodyClick: false,
    });

    return await toggle.open();
  },

  showSuccess: showFromTemplate("#tpl-success-notification"),
  showError: showFromTemplate("#tpl-error-notification"),
};

function init(registry: ControlRegistry) {
  Notifications.init();

  registry.registerGlobalApi({
    showSuccessNotification(message: string) {
      Notifications.showSuccess({
        kind: "SUCCESS",
        message,
      });
    },
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
