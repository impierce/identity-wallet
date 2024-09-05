import {
  DownloadSimpleFillIcon,
  FileArrowDownFillIcon,
  KeyFillIcon,
  LockSimpleOpenFillIcon,
  TrophyFillIcon,
  UserCirclePlusFillIcon,
} from '$lib/icons';

export const icons = {
  Trophy: TrophyFillIcon,
  DownloadSimple: DownloadSimpleFillIcon,
  LockSimpleOpen: LockSimpleOpenFillIcon,
  UserCirclePlus: UserCirclePlusFillIcon,
  FileArrowDown: FileArrowDownFillIcon,
  Key: KeyFillIcon,
} as const;

export type Icon = keyof typeof icons;
