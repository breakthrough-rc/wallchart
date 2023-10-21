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
    document.body.addEventListener("yc:notificationRequest", (ev: Event) => {
      const request = (ev as CustomEvent<NotificationRequest>).detail;
      switch (request.kind) {
        case "ERROR":
          Notifications.showError(request);
          break;
        case "GENERIC":
          break;
      }
    });
  },

  appendNotification(notification: HTMLElement) {
    Notifications.content.appendChild(notification);
  },

  showError(request: ErrorNotificationRequest) {
    const tpl = document.getElementById("tpl-error-notification") as HTMLTemplateElement;
    if (!tpl) throw new Error("Can not show ErrorNotification. Element with id `tpl-error-notification` not found.");

    const notification = tpl.content.cloneNode(true) as HTMLElement;
    const messageElement = notification.querySelector("[data-error-message]");
    if (!messageElement) throw new Error("Could not find element with attribute `data-error-message` in template.");

    messageElement.textContent = request.message || "Unknown error";
    Notifications.appendNotification(notification);
  },
};

function init() {
  Notifications.init();
}

export default {
  init,
}
