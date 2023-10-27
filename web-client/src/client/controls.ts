import registry from "./controls/registery";
import events from "./controls/events";
import Notifications from "./controls/Notification";
import Toggle from "./controls/Toggle";

function init() {
  registry.init();

  events.init(registry);
  Notifications.init(registry);
  Toggle.init(registry);

  registry.ready();
}

export default {
  init,
}
