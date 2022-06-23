import { isArray } from "./../../shared/src/index";
import { isString, ShapeFlags } from "@vue/shared";

export function isVnode(value) {
  return !!(value && value.__v_isVnode);
}

export function createVnode(type, props, children = null) {
  // 组合方案 shapeFlag
  let shapeFlag = isString(type) ? ShapeFlags.ELEMENT : 0;

  const vnode = {
    type,
    props,
    children,
    el: null, // 对应的真实节点
    key: props?.["key"],
    __v_isVnode: true,
    shapeFlag,
  };

  if (children) {
    let type = 0;
    if (isArray(children)) {
      type = ShapeFlags.ARRAY_CHILDREN;
    } else {
      children = String(children);
      type = ShapeFlags.TEXT_CHILDREN;
    }
    vnode.shapeFlag |= type;
  }
  return vnode;
}
