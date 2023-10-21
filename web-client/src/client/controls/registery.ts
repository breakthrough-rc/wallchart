export type ControlRegistry = {
  registerGlobalApi(api: any): void,
};

function init() {
  (window as any).YcControls = {};
}

function registerGlobalApi(api: any) {
  (window as any).YcControls = {
    ...(window as any).YcControls,
    ...api,
  }
}

export default {
  init,
  registerGlobalApi,
}
