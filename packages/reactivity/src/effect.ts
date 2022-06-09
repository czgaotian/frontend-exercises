export let activeEffect = undefined;

class ReactiveEffect {
  public parent = null;
  // 记录依赖其的属性
  public deps = [];
  public active = true;
  constructor(public fn) {}

  run() {
    if (!this.active) {
      this.fn();
    }

    // 依赖收集
    try {
      this.parent = activeEffect;
      activeEffect = this;
      this.fn();
    } finally {
      activeEffect = this.parent;
      this.parent = null;
    }
  }
}

export const effect = (fn) => {
  const _effect = new ReactiveEffect(fn);

  _effect.run();
};

const targetMap = new WeakMap();
export const track = (target, type, key) => {
  if (!activeEffect) return;
  let depsMap = targetMap.get(target);
  if (!depsMap) {
    targetMap.set(target, (depsMap = new Map()));
  }
  let dep = depsMap.get(key);
  if (!dep) {
    depsMap.set(key, (dep = new Set()));
  }
  let shouldTrack = !dep.has(activeEffect);
  if (shouldTrack) {
    dep.add(activeEffect);
    // 存放属性对应的Set<ReactiveEffect>
    activeEffect.deps.push(dep);
  }
};

export const trigger = (target, type, key, newValue, oldValue) => {
  const depsMap = targetMap.get(target);
  if (!depsMap) return;
  const effects = depsMap.get(key);
  effects &&
    effects.forEach((effect) => {
      if (effect.active) {
        if (effect !== activeEffect) effect.run();
      }
    });
};
