import { writable } from 'svelte/store';

/**
 * Carries the chosen backup and its password between the password screen and
 * the preview screen, so the user is only asked once.
 *
 * Scoped to the recovery flow and cleared as soon as the restore completes or
 * the user leaves. Mirrors how `onboarding_state` holds the profile password
 * during onboarding.
 */
export const recovery = writable<{ id: string; password: string } | null>(null);
