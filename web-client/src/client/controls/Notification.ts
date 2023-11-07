import events from "../events";
import { ControlRegistry } from "../registery";
import Toggle from "./Toggle";

type NotificationEventDetails
  = { kind: 'SUCCESS' | 'ERROR' | 'GENERIC' }
  & Partial<NotificationCommand>;

type NotificationCommand = {
  title: string,
  message: string,
}

type ShowDelegate = {
  provideIconElement?: () => Element | undefined,
  notificationWillAppend?: (notificationEl: HTMLElement) => void,
};

const nullDelegate: ShowDelegate = {};

let renderStandardTemplate = async ({ title, message, ...delegate }: NotificationCommand & ShowDelegate) => {
  const tpl = document.querySelector("#tpl-notification") as HTMLTemplateElement;
  if (!tpl) throw new Error(`Can not show Notification. Element selector "#tpl-notification" not found.`);

  const notification = tpl.content.cloneNode(true) as HTMLElement;

  const titleElement = notification.querySelector("[data-notification-title]");
  if (!titleElement) throw new Error("Could not find element with attribute `data-notification-title` in template.");
  titleElement.textContent = title;

  const messageElement = notification.querySelector("[data-notification-message]");
  if (!messageElement) throw new Error("Could not find element with attribute `data-notification-message` in template.");
  messageElement.textContent = message || "Everything is all good!";

  const providedIconElement = delegate.provideIconElement?.();
  if (providedIconElement) {
    const defaultIconElement = notification.querySelector("[data-notification-icon]");
    if (!defaultIconElement) throw new Error("Could not find element with attribute `data-notification-icon` in template.");
    defaultIconElement.replaceWith(providedIconElement);
  }

  delegate.notificationWillAppend?.(notification);
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
    events.on("yc:notificationRequest", (request: NotificationEventDetails) => {
      const { title = "Notification", message = "", } = request;
      switch (request.kind) {
        case "SUCCESS":
          Notifications.showSuccess(message);
          break;
        case "ERROR":
          Notifications.showError(message);
          break;
        case "GENERIC":
          Notifications.show(title, message);
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

  show: (title: string, message: string) => renderStandardTemplate({ title, message }),

  showSuccess: (message: string) => renderStandardTemplate({
    title: "Success!",
    message,
    provideIconElement() {
      const tpl = document.querySelector("#tpl-notification-icons") as HTMLTemplateElement;
      if (!tpl) throw new Error("Could not find element with selector `#tpl-notification-icons` in template.");

      const iconsTemplate = tpl.content.cloneNode(true) as HTMLElement;

      const infoIcon = iconsTemplate.querySelector("[data-notification-icon=success]");
      if (!infoIcon) throw new Error("Could not find element with attribute `data-notification-icon=success` in template.");
      return infoIcon;
    },
  }),

  showError: (message: string) => renderStandardTemplate({
    title: "Oops! Something went wrong",
    message,
    provideIconElement() {
      const tpl = document.querySelector("#tpl-notification-icons") as HTMLTemplateElement;
      if (!tpl) throw new Error("Could not find element with selector `#tpl-notification-icons` in template.");

      const iconsTemplate = tpl.content.cloneNode(true) as HTMLElement;

      const infoIcon = iconsTemplate.querySelector("[data-notification-icon=error]");
      if (!infoIcon) throw new Error("Could not find element with attribute `data-notification-icon=success` in template.");
      return infoIcon;
    },
  }),
};

function init(registry: ControlRegistry) {
  Notifications.init();

  registry.registerGlobalApi({
    showNotification: Notifications.show,
    showSuccessNotification: Notifications.showSuccess,
    showErrorNotification: Notifications.showError,
  });
}

export default {
  init,
}
