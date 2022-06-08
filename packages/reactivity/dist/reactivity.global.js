(() => {
  // packages/reactivity/src/effect.ts
  var activeEffect = void 0;
  var createReactiveEffect = class {
    constructor(fn) {
      this.fn = fn;
      this.active = true;
    }
    run() {
      if (!this.active) {
        this.fn();
      }
      try {
        activeEffect = this;
        this.fn();
      } finally {
        activeEffect = void 0;
      }
    }
  };
  var effect = (fn) => {
    const _effect = new createReactiveEffect(fn);
    _effect.run();
  };

  // packages/shared/src/index.ts
  var isObject = (value) => {
    return typeof value === "object" && value !== null;
  };

  // packages/reactivity/src/baseHandler.ts
  var mutableHandlers = {
    get(target, key, receiver) {
      if (key === "__v_isReactive" /* IS_REACTIVE */) {
        return true;
      }
      return Reflect.get(target, key, receiver);
    },
    set(target, key, value, receiver) {
      return Reflect.set(target, key, value, receiver);
    }
  };

  // packages/reactivity/src/reactive.ts
  var reactiveMap = /* @__PURE__ */ new WeakMap();
  var reactive = (target) => {
    if (isObject(target)) {
      return;
    }
    if (target["__v_isReactive" /* IS_REACTIVE */]) {
      return target;
    }
    let existingProxy = reactiveMap.get(target);
    if (existingProxy) {
      return existingProxy;
    }
    const proxy = new Proxy(target, mutableHandlers);
    reactiveMap.set(target, proxy);
    return proxy;
  };
})();
