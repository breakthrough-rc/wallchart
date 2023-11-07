import events from "../events";
import { ControlRegistry } from "../registery";
import Toggle from "./Toggle";

type NotificationRequest = {
  kind: 'SUCCESS' | 'ERROR' | 'GENERIC',
  title?: string,
  message: string,
};

type RenderDelegate = {
  provideIconElement?: (request: NotificationRequest) => Element | undefined,
  notificationWillAppend?: (notificationEl: HTMLElement, request: NotificationRequest) => void,
};

const nullDelegate: RenderDelegate = {};

let showFromTemplate = (delegate: RenderDelegate = nullDelegate) => async (request: NotificationRequest) => {
  const tpl = document.querySelector("#tpl-notification") as HTMLTemplateElement;
  if (!tpl) throw new Error(`Can not show Notification. Element selector "${tplSelector}" not found.`);

  const notification = tpl.content.cloneNode(true) as HTMLElement;

  if (request.title) {
    const titleElement = notification.querySelector("[data-notification-title]");
    if (!titleElement) throw new Error("Could not find element with attribute `data-notification-title` in template.");
    titleElement.textContent = request.title;
  }

  const messageElement = notification.querySelector("[data-notification-message]");
  if (!messageElement) throw new Error("Could not find element with attribute `data-notification-message` in template.");
  messageElement.textContent = request.message || "Everything is all good!";

  const providedIconElement = delegate.provideIconElement?.(request);
  if (providedIconElement) {
    const defaultIconElement = notification.querySelector("[data-notification-icon]");
    if (!defaultIconElement) throw new Error("Could not find element with attribute `data-notification-icon` in template.");
    defaultIconElement.replaceWith(providedIconElement);
  }

  delegate.notificationWillAppend?.(notification, request);
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

  show: showFromTemplate(),
  showSuccess: showFromTemplate({
    provideIconElement(_: NotificationRequest) {
      const tpl = document.querySelector("#tpl-notification-icons") as HTMLTemplateElement;
      if (!tpl) throw new Error("Could not find element with selector `#tpl-notification-icons` in template.");

      const iconsTemplate = tpl.content.cloneNode(true) as HTMLElement;

      const infoIcon = iconsTemplate.querySelector("[data-notification-icon=success]");
      if (!infoIcon) throw new Error("Could not find element with attribute `data-notification-icon=success` in template.");
      return infoIcon;
    },
  }),
  showError: showFromTemplate({
    provideIconElement(_: NotificationRequest) {
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
    showNotification(title: string, message: string, icon?: string | HTMLElement) {
      Notifications.show({
        kind: "GENERIC",
        title,
        message,
      });
    },
    showSuccessNotification(message: string) {
      Notifications.showSuccess({
        kind: "SUCCESS",
        title: "Success",
        message,
      });
    },
    showErrorNotification(message: string) {
      Notifications.showError({
        kind: "ERROR",
        title: "Oops! Somethingn went wrong",
        message,
      });
    },
  });
}

export default {
  init,
}
