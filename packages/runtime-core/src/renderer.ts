import { ShapeFlags } from "@vue/shared";
export function createRenderer(renderOptions) {
  let {
    insert: hostInsert,
    remove: hostRemove,
    setElementText: hostSetElementText,
    setText: hostSetText,
    querySelector: hostQuerySelector,
    parentNode: hostParentNode,
    nextSibling: hostNextSibling,
    createElement: hostCreateElement,
    createText: hostCreateText,
    patchProp: hostPatchProp,
  } = renderOptions;

  const mountChildren = (children, container) => {
    for (let i = 0; i < children.length; i++) {
      patch(null, children[i], container);
    }
  };

  const mountElement = (vnode, container) => {
    let { type, props, children, shapeFlag } = vnode;
    // 真实元素挂载到虚拟节点
    let el = (vnode.el = hostCreateElement(type));
    if (props) {
      for (let key in props) {
        hostPatchProp(el, key, props[key]);
      }
    }
    if (shapeFlag & ShapeFlags.TEXT_CHILDREN) {
      // 文本节点
      hostSetElementText(el, children);
    } else if (shapeFlag & ShapeFlags.ARRAY_CHILDREN) {
      mountChildren(children, el);
    }
    hostInsert(el, container);
  };

  const patch = (n1, n2, container) => {
    if (n1 === n2) return;

    if (n1 === null) {
      // 初次渲染
      mountElement(n2, container);
    } else {
      // 更新流程
    }
  };

  const render = (vnode, container) => {
    if (vnode == null) {
      // 当前vnode是空，卸载
    } else {
      // 挂载，初始化和更新
      patch(null, vnode, container);
    }
  };
  return {
    render,
  };
}
