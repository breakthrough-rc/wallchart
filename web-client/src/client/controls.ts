import registry from "./registery";
import events from "./events";
import Notifications from "./controls/Notification";
import Toggle from "./controls/Toggle";

const global = (window as any);

function create(attachOnReadyQueue: HTMLElement[] = [], onReadyQueue: CallableFunction[] = []) {
  const controls = {
    ready() {
      attachOnReadyQueue.forEach(controls.attach);
      onReadyQueue.forEach(cb => cb());
    },
    onReady(cb: CallableFunction) {
      cb(); // At this point, YcControls is already loaded and ready, just call back.
    },
    attach(element: HTMLElement) {
      console.log("[YcControls::attach()]", element);
      const controlKey = element.dataset.ycControl;

      if (!controlKey) {
        console.warn("Element has no `data-yc-control` attribute. Skipping.");
        return;
      }

      const control = registry.get(controlKey);
      if (!control) {
        console.warn(`No control found for key: ${controlKey}. Skipping.`);
        return;
      }

      control.attach(element);
    },
  };

  return controls;
}

function init() {
  const YcControls = global.YcControls = create(global.YcControls.attachOnReadyQueue, global.YcControls.onReadyQueue);

  events.init(registry);
  Notifications.init(registry);
  Toggle.init(registry);

  YcControls.ready();
}

export default {
  init,
}
