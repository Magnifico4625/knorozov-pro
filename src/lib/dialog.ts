// Keep keyboard focus inside a modal, then return it to the control that opened it.
export function modalDialog(node: HTMLElement, close: () => void) {
  const previous = document.activeElement instanceof HTMLElement ? document.activeElement : null;
  const controls = () => Array.from(node.querySelectorAll<HTMLElement>(
    'button:not(:disabled), input:not(:disabled), select:not(:disabled), textarea:not(:disabled), a[href], [tabindex="0"]',
  )).filter((el) => el.getClientRects().length > 0);

  function onKey(e: KeyboardEvent) {
    if (e.key === "Escape") {
      e.preventDefault();
      e.stopPropagation();
      close();
    } else if (e.key === "Tab") {
      const items = controls();
      const first = items[0];
      const last = items[items.length - 1];
      if (!first) {
        e.preventDefault();
        node.focus();
      } else if (e.shiftKey && document.activeElement === first) {
        e.preventDefault();
        last.focus();
      } else if (!e.shiftKey && document.activeElement === last) {
        e.preventDefault();
        first.focus();
      }
    }
  }

  node.addEventListener("keydown", onKey);
  queueMicrotask(() => (controls()[0] ?? node).focus());
  return {
    destroy() {
      node.removeEventListener("keydown", onKey);
      if (previous?.isConnected) previous.focus();
    },
  };
}
