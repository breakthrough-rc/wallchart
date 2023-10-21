# 3. UI Controls

Date: 2023-10-22

## Status

Accepted

## Context

Our approach to building this web application is to rely on our backend framework to provide server-side rendering.
For common client side logic, we are leveraging HTMX for tasks such as fetching content from the server, updating the dom, etc.

Unfortunately, there are limits to what HTMX can provide out of the box (per design). To provide a rich interactive user interface, we will need a way to add behavior, via JavaScript, to UI components. Since we are using HTMX and not a framework/library like React and Vue, we will need to take a different approach to adding client side logic.

One concrete example is being able to leverage the [TailwindUI component library](https://tailwindui.com/) for common UI components such as Notifications, Tooltips, etc. TailwindUI will provide JS with React and Vue components. Since we are not using these frameworks, we need a mechanism to add JavaScript for dynamic behavior and client-side logic (toggling elements, classes for animations and transitions, etc).

## Decision

* [TailwindUI component library](https://tailwindui.com/)
* Controls will be written in vanilla TypeScript inside the `web-client` package. No framework or library is required.
* To hook into css animations and transitions we will use the [el-transition npm package](https://www.npmjs.com/package/el-transition). This package will allow us to declaratively set classes for `enter, enter to, enter from, leave, leave to, and leave from` hooks when applying css transitions.
* Expose API on `window.YcControls` for client and server components to listen and trigger events and access global control APIs.
* Leverage the standard DOM event system via [CustomEvents](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent), similar to HTMX, for controls to communicate and to trigger behavior (show, hide, and custom behaviors). This includes listening to HTMX event and republishing or directly using API on `YcControls`.

### Controls will be defined in the `web-client` package 
The `web-client` package will be the home for client side custom controls (The term `custom control` here means an HTML element, which is rendered by a server side or client side component, that has been enhanced with additional behavior and functionality).

Custom controls will live inside this package's `src/client/controls` folder. 
Each module will need to export an `init` function. 
This function can be used to listen for `CustomEvent` off the `document.body`.
The `init` function will also be passed a register to extend the `YcControls` global object.

The objective is to allow the `web-htmx` to render HTML server-side and when needing to return client side logic can do so in a declarative manner using a very simple API.

We should monitor how JavaScript is written in the `web-htmx` package - If a lot of code is written and begins to take a more imperative shape, chances are we are missing a concept/control in `web-client`.

### Simple Example: Foo control

This control will simply `console.log` a message.

```typescript
// Inside web-htmx package...
// Code that triggers the event. This could be declared in rsx component in Rust.
<button onclick="YcControls.trigger('yc:fooRequest', { foo: 'baz' })";
  Click to see `foo` work its magic!
</button>

// Inside `web-client`` package
// Foo control in /src/client/controls/Foo.ts
import { ControlRegistry } from "./registery";

type FooRequest = {
  foo: string,
};

function init(registry: ControlRegistry) {
  // Optional if you like to expose global api for logging 
  // This exposes a global fn on `window.YcControls.fooLog`
  registry.registerGlobalApi({ fooLog });

  document.body.addEventListener("yc:fooRequest", (ev: Event) => {
    const request = (ev as CustomEvent<FooRequest>).detail;
    log(request);
  };
}

function fooLog(request: FooRequest) {
  console.log("Foo!", request.foo);
}

export default {
  init,
}
```

## Consequences

The descisions outlined in this ADR might work for simple use cases and errs on the side of a simpler more light-weight approach. We might find use cases that require more complex interactions or state management to name a few. We also could see a lot of overhead needed to write common DOM interactions.

### Not All BeSpoke JavaScript Might Not Be a Control
This decision does not provide any support other than what is exposed on `YcControls` global object for writting JavaScript when writing Rust server side components. 

If we find the code can not be abstracted into a web-client control, and the bespoke JavaScript is a valid use case, we might want to look a library such as [hyperscript](https://hyperscript.org/) to enforce a declarative, readable, conscise syntax for common custom client side behavior inside our Rust components.

### Vanilla TypeScript Controls
The decision to define controls w/o a framework might not scale. We migh need state management or need to support more complex behaviors.

If we find his to be true in the future, we should look at the web component standard and libraries that provide sugar for defining components such as [HybridJs](https://hybrids.js.org/#/) and [Lit](https://lit.dev/).

Additionally or alternatively, if we find the Vanilla TS approach not scaling, we might want to explore a library like [AlpineJs](https://alpinejs.dev/) for defining controls.
