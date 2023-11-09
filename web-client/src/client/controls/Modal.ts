import { ControlRegistry } from "../registery";

function init(registry: ControlRegistry) {
  registry.registerGlobalApi({
    openModal: () => alert('openModal::coming soon!'),
  });
}

export default {
  init,
}