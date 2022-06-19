// packages/runtime-dom/src/nodeOps.ts
var nodeOps = {
  insert(child, parent, anchor = null) {
    if (anchor) {
      parent.insertBefore(child, anchor);
    } else {
      parent.appendChild(child);
    }
  },
  remove(child) {
    const parentNode = child.parentNode;
    if (parentNode) {
      parentNode.removeChild(child);
    }
  },
  setElementText(el, text) {
    el.textContent = text;
  },
  setText(node, text) {
    node.nodeValue = text;
  },
  querySelector(selector) {
    return document.querySelector(selector);
  },
  parentNode(node) {
    return node.parentNode;
  },
  nextSibling(node) {
    return node.nextSibling;
  },
  createElement(tagName) {
    return document.createElement(tagName);
  },
  createText(text) {
    return document.createTextNode(text);
  }
};

// packages/runtime-dom/src/patchProps.ts
function patchProp(el, key, preValue, nextValue) {
}

// packages/runtime-dom/src/index.ts
var renderOptions = Object.assign(nodeOps, { patchProp });
