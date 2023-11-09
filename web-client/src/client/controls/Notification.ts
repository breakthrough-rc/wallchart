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

type RenderTemplateDelegate = {
  iconElement?: Element | undefined,
  notificationWillAppend?: (notificationEl: HTMLElement) => void,
};

const nullDelegate: RenderTemplateDelegate = {};

function renderStandardTemplate({ title, message, ...delegate }: NotificationCommand & RenderTemplateDelegate) {
  const tpl = document.querySelector("#tpl-notification") as HTMLTemplateElement;
  if (!tpl) throw new Error(`Can not show Notification. Element selector "#tpl-notification" not found.`);

  const notification = tpl.content.cloneNode(true) as HTMLElement;

  const titleElement = notification.querySelector("[data-notification-title]");
  if (!titleElement) throw new Error("Could not find element with attribute `data-notification-title` in template.");
  titleElement.textContent = title;

  const messageElement = notification.querySelector("[data-notification-message]");
  if (!messageElement) throw new Error("Could not find element with attribute `data-notification-message` in template.");
  messageElement.textContent = message || "Everything is all good!";

  if (delegate.iconElement) {
    const defaultIconElement = notification.querySelector("[data-notification-icon]");
    if (!defaultIconElement) throw new Error("Could not find element with attribute `data-notification-icon` in template.");
    defaultIconElement.replaceWith(delegate.iconElement);
  }

  delegate.notificationWillAppend?.(notification);
  return Notifications.appendNotification(notification);
};

function renderCustomTemplate(templateSelector: string | HTMLTemplateElement) {
  const tpl = (() => {
    if (typeof templateSelector === "string") {
      const template = document.querySelector(templateSelector) as HTMLTemplateElement;
      if (!template) throw new Error(`Could not find template with selector: ${templateSelector}`);

      return template;

    } else if (templateSelector instanceof HTMLTemplateElement) {
      return templateSelector;

    } else {
      throw new Error("Invaliad argument. Expected selector string or HTMLTemplateElement.");
    }
  })();

  const notification = tpl.content.cloneNode(true) as HTMLElement;

  // Ensure notification is clickable (live-region disables pointer events)
  if (!notification.firstElementChild?.classList.contains("pointer-events-auto")) {
    notification.firstElementChild?.classList.add("pointer-events-auto");
  }

  return Notifications.appendNotification(notification);
}

function iconFromTemplate(iconKey: "success" | "error" | "info") {
  const tpl = document.querySelector("#tpl-notification-icons") as HTMLTemplateElement;
  if (!tpl) throw new Error("Could not find element with selector `#tpl-notification-icons`.");

  const iconsTemplate = tpl.content.cloneNode(true) as HTMLElement;

  const iconElement = iconsTemplate.querySelector(`[data-notification-icon=${iconKey}]`);
  if (!iconElement) throw new Error("Could not find element with selector: `[data-notification-icon=${iconKey}]`.");
  return iconElement;
}

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
    iconElement: iconFromTemplate("success"),
  }),

  showError: (message: string) => renderStandardTemplate({
    title: "Oops! Something went wrong",
    message,
    iconElement: iconFromTemplate("error"),
  }),
};

function init(registry: ControlRegistry) {
  Notifications.init();

  registry.registerGlobalApi({
    showNotification: Notifications.show,
    showSuccessNotification: Notifications.showSuccess,
    showErrorNotification: Notifications.showError,
    showNotificationWithTemplate: renderCustomTemplate,
  });
}

export default {
  init,
}
