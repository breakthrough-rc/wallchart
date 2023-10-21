import registry from "./controls/registery";
import Notifications from "./controls/Notification";

type YcEventDetail = any;

/**
 * event name should begin with prefix `yc:`
 */
function trigger(event: string, detail: YcEventDetail = {}) {
  document.body.dispatchEvent(new CustomEvent(event, { detail }));
}

function init() {
  registry.init();

  registry.registerGlobalApi({
    trigger,
  });

  Notifications.init(registry);
}

export default {
  init,
}
