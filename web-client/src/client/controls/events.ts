import { ControlRegistry } from "./registery";

type YcEventDetail = any;

/**
 * event name should begin with prefix `yc:`
 */
function trigger(event: string, detail: YcEventDetail = {}) {
  document.body.dispatchEvent(new CustomEvent(event, { detail }));
}

function init(register: ControlRegistry) {
  register.registerGlobalApi({
    trigger,
  });
}

export default {
  init,
}
