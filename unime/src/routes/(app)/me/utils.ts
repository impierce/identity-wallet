export const calculateInitials = (name: string): string => {
  const parts = name.split(' ').filter((n) => n.length > 0);
  if (parts.length === 1) {
    // Take first two letters, if only one name
    // eslint-disable-next-line @typescript-eslint/no-extra-non-null-assertion
    return parts.at(0)!!.slice(0, 2).toUpperCase();
  } else {
    const first = parts?.at(0)?.charAt(0) ?? '?';
    const last = parts?.at(1)?.charAt(0) ?? '?';
    return `${first}${last}`.toUpperCase();
  }
};
