import { ControlRegistry } from "./registery";

type YcEventDetail = any;
type YcEventHandler = (detail: YcEventDetail, originalEvent: CustomEvent) => void;

/**
 * event name should begin with prefix `yc:`
 */

function trigger(event: string, detail: YcEventDetail = {}) {
  document.body.dispatchEvent(new CustomEvent(event, { detail }));
}

function on(eventName: string, handler: YcEventHandler) {
  function cb(event: Event) {
    const detail = (event as CustomEvent).detail;
    return handler(detail, event as CustomEvent);
  }

  document.body.addEventListener(eventName, cb);

  return () => {
    document.body.removeEventListener(eventName, cb);
  };
}

function init(register: ControlRegistry) {
  register.registerGlobalApi({
    on,
    trigger,
  });
}

export default {
  init,
  on,
  trigger,
}
