import { shareText } from '@choochmeque/tauri-plugin-sharekit-api';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { platform } from '@tauri-apps/plugin-os';

import { shareOrCopyLink } from './share';

vi.mock('@choochmeque/tauri-plugin-sharekit-api', () => ({ shareText: vi.fn() }));
vi.mock('@tauri-apps/plugin-clipboard-manager', () => ({ writeText: vi.fn() }));
vi.mock('@tauri-apps/plugin-os', () => ({ platform: vi.fn() }));

const text = 'Bachelor from TU Delft: https://example.com/public/verify/1';
const link = 'https://example.com/public/verify/1';

describe('shareOrCopyLink', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(platform).mockReturnValue('android');
  });

  test('opens the share sheet with the text', async () => {
    expect(await shareOrCopyLink(text, link)).toBe('shared');
    expect(shareText).toHaveBeenCalledWith(text);
    expect(writeText).not.toHaveBeenCalled();
  });

  test('treats a dismissed share sheet as shared', async () => {
    vi.mocked(shareText).mockRejectedValue('Share cancelled');
    expect(await shareOrCopyLink(text, link)).toBe('shared');
    expect(writeText).not.toHaveBeenCalled();
  });

  test('copies the link when the share sheet fails', async () => {
    vi.mocked(shareText).mockRejectedValue('Windows API error: Class not registered');
    expect(await shareOrCopyLink(text, link)).toBe('copied');
    expect(writeText).toHaveBeenCalledWith(link);
  });

  test('copies the link on Linux without trying the share sheet', async () => {
    vi.mocked(platform).mockReturnValue('linux');
    expect(await shareOrCopyLink(text, link)).toBe('copied');
    expect(shareText).not.toHaveBeenCalled();
    expect(writeText).toHaveBeenCalledWith(link);
  });
});
