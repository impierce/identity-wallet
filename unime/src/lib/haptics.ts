import { isTauri } from '@tauri-apps/api/core';
import {
  impactFeedback,
  notificationFeedback,
  selectionFeedback,
  type ImpactFeedbackStyle,
} from '@tauri-apps/plugin-haptics';
import { warn } from '@tauri-apps/plugin-log';
import { platform } from '@tauri-apps/plugin-os';

// iOS only for now. On Android the haptics plugin plays raw vibration waveforms rather than the system's haptic
// effects, which feel buzzy on many devices, most of all those without amplitude control.
let enabled: boolean | undefined;

function isEnabled(): boolean {
  if (enabled === undefined) {
    try {
      enabled = isTauri() && platform() === 'ios';
    } catch {
      enabled = false;
    }
  }
  return enabled;
}

function play(effect: () => Promise<unknown>) {
  if (!isEnabled()) return;
  effect().catch((e) => warn(`Haptic feedback failed: ${e}`));
}

/**
 * Haptic feedback for a few meaningful moments. Sparing on purpose: a haptic that follows every tap stops meaning
 * anything. iOS leaves them out when the user turned off "System Haptics".
 */
export const haptics = {
  /** A value was toggled or picked, such as marking a credential as a favorite. */
  selection: () => play(selectionFeedback),
  /** Something physical happened, such as a QR code being detected. */
  impact: (style: ImpactFeedbackStyle = 'medium') => play(() => impactFeedback(style)),
  /** A task the user started completed, such as a credential being accepted. */
  success: () => play(() => notificationFeedback('success')),
  /** A task the user started failed. */
  error: () => play(() => notificationFeedback('error')),
};
