const global = (window as any);

export type ControlRegistry = {
  registerGlobalApi(api: any): void,
  registerControl(controlKey: string, control: any): void,
};

let attachOnReadyQueue: HTMLElement[] = [];
let controls = new Map<string, any>();

const YcControls_ = {
  attach(element: HTMLElement) {
    console.log("[YcControls::attach()]", element);
    const controlKey = element.dataset.ycControl;

    if (!controlKey) {
      console.warn("Element has no `data-yc-control` attribute. Skipping.");
      return;
    }

    const control = controls.get(controlKey);
    if (!control) {
      console.warn(`No control found for key: ${controlKey}. Skipping.`);
      return;
    }

    control.attach(element);
  },
};

function init() {
  attachOnReadyQueue = global.YcControls.attachOnReadyQueue || [];
  global.YcControls = YcControls_;
}

function ready() {
  attachOnReadyQueue.forEach(YcControls_.attach);
}

function registerControl(controlKey: string, control: any) {
  controls.set(controlKey, control);
}

function registerGlobalApi(api: any) {
  global.YcControls = {
    ...global.YcControls,
    ...api,
  }
}

export default {
  init,
  ready,
  registerControl,
  registerGlobalApi,
}
