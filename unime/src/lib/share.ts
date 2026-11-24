import { shareText } from '@choochmeque/tauri-plugin-sharekit-api';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { platform } from '@tauri-apps/plugin-os';

/**
 * `shared`: the native share sheet was shown (the user may have cancelled it).
 * `copied`: there is no share sheet on this platform, or it failed, so the link was copied to the clipboard instead.
 */
export type ShareOutcome = 'shared' | 'copied';

/**
 * Opens the native share sheet with `text`. Linux has no share sheet, so there `link` is copied to the clipboard, and
 * the same happens when the share sheet fails to open.
 */
export async function shareOrCopyLink(text: string, link: string): Promise<ShareOutcome> {
  if (platform() !== 'linux') {
    try {
      await shareText(text);
      return 'shared';
    } catch (error) {
      // Android and Windows report a dismissed share sheet as an error.
      if (String(error).toLowerCase().includes('cancel')) {
        return 'shared';
      }
    }
  }
  await writeText(link);
  return 'copied';
}
