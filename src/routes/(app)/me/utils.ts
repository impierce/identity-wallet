// TODO: remove
export const calculate_initials = (name: string): string => {
  let names = name.split(' ');
  if (names.length === 1) {
    return names.at(0)!!.slice(0, 2).toUpperCase();
  } else {
    let first = names?.at(0)?.charAt(0) ?? '?';
    let last = names?.at(1)?.charAt(0) ?? '?';
    // initials = names?.at(0)?.charAt(0) ?? '' + names?.at(1)?.charAt(0) ?? '';
    return first + '' + last;
  }
};
