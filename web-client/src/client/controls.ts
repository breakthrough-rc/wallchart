import registry from "./registery";
import events from "./events";
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
