export const calculateInitials = (name: string): string => {
  let parts = name.split(' ').filter((n) => n.length > 0);
  if (parts.length === 1) {
    // Take first two letters, if only one name
    return parts.at(0)!!.slice(0, 2).toUpperCase();
  } else {
    let first = parts?.at(0)?.charAt(0) ?? '?';
    let last = parts?.at(1)?.charAt(0) ?? '?';
    // initials = names?.at(0)?.charAt(0) ?? '' + names?.at(1)?.charAt(0) ?? '';
    return `${first}${last}`.toUpperCase();
  }
};
