import type { PinInput } from 'melt/builders';

const ALLOWED: Record<PinInput['type'], RegExp> = {
  numeric: /[0-9]/,
  alphanumeric: /[a-zA-Z0-9]/,
  text: /\S/,
};

/**
 * The `onPaste` of a melt `PinInput`, which melt calls for a paste and for any input of more than one character at once
 * (autofill, a keyboard suggestion). Spreads the characters the input type allows over the boxes, from the focused box
 * on (from the first, for a whole code), and moves the focus to the box after the last one filled. melt itself leaves
 * such input in the one box it was entered in; setting `value` puts every box back to a single character.
 */
export function fillPinInput(pin: PinInput, text: string) {
  const inputs = [
    ...(document.getElementById(pin.root.id)?.querySelectorAll<HTMLInputElement>('[data-melt-pin-input-input]') ?? []),
  ];
  const focused = Math.max(inputs.indexOf(document.activeElement as HTMLInputElement), 0);

  // No new text: the box was emptied without a Backspace key event, as some mobile keyboards do.
  if (!text) {
    pin.value = pin.value.slice(0, focused) + pin.value.slice(focused + 1);
    return;
  }

  const chars = [...text].filter((char) => ALLOWED[pin.type].test(char)).join('');
  const start = chars.length >= pin.maxLength ? 0 : Math.min(focused, pin.value.length);
  pin.value = (pin.value.slice(0, start) + chars + pin.value.slice(start + chars.length)).slice(0, pin.maxLength);
  inputs[Math.min(start + chars.length, pin.maxLength - 1)]?.focus();
}
