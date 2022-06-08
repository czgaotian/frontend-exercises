export let activeEffect = undefined;

class createReactiveEffect {
  public active = true;
  constructor(public fn){

  }

  run() {
    if (!this.active) {
      this.fn();
    }

    // 依赖收集
    try {
      activeEffect = this;
      this.fn();
    } finally {
      activeEffect = undefined;
    }
  }
}

export const effect = (fn) => {
  const _effect = new createReactiveEffect(fn);

  _effect.run()
}