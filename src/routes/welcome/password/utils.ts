export const passwordPolicy = [
  {
    name: 'uppercase letter',
    regex: /[A-Z]/,
    count: 1,
  },
  {
    name: 'lowercase letter',
    regex: /[a-z]/,
    count: 1,
  },
  {
    name: 'number',
    regex: /[0-9]/,
    count: 1,
  },
  {
    name: 'characters',
    regex: /.{8,}/,
    count: 8,
  },
];

export const checkPasswordPolicy = (password: string) => {
  const violations: string[] = [];
  passwordPolicy.forEach((rule) => {
    if (!password.match(rule.regex)) {
      console.warn(`Password does not match: ${rule.name}`);
      violations.push(rule.name);
    }
  });
  return violations;
};
