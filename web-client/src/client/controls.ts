import registry from "./controls/registery";
import events from "./controls/events";
import Notifications from "./controls/Notification";

function init() {
  registry.init();

  events.init(registry);
  Notifications.init(registry);
}

export default {
  init,
}
