import DownloadSimple from '~icons/ph/download-simple-fill';
import FileArrowDown from '~icons/ph/file-arrow-down-fill';
import Key from '~icons/ph/key-fill';
import LockSimpleOpen from '~icons/ph/lock-simple-open-fill';
import Trophy from '~icons/ph/trophy-fill';
import UserCirclePlus from '~icons/ph/user-circle-plus-fill';

export const icons = {
  Trophy: Trophy,
  DownloadSimple: DownloadSimple,
  LockSimpleOpen: LockSimpleOpen,
  UserCirclePlus: UserCirclePlus,
  FileArrowDown: FileArrowDown,
  Key: Key,
} as const;

export type Icon = keyof typeof icons;
