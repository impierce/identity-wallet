// Ensures the usage of the light variants of predefined icons.
const ICONS = ['Bank', 'Certificate', 'User', 'IdentificationBadge', 'House'] as const;
const LIGHT_ICONS = ['BankLight', 'CertificateLight', 'UserLight', 'IdentificationBadgeLight', 'HouseLight'] as const;

type Icon = (typeof ICONS)[number];
type LightIcon = (typeof LIGHT_ICONS)[number];

export const ensureLightIcon = (icon: string | undefined): LightIcon | undefined => {
  // Converts the passed icon to the light version, if applicable.
  if (!icon) return undefined;
  if (!ICONS.includes(icon as Icon) && !LIGHT_ICONS.includes(icon as LightIcon)) {
    return undefined;
  }
  if (icon.endsWith('Light')) {
    return icon as LightIcon; // Already a light icon
  } else {
    return `${icon}Light` as LightIcon; // Convert to light icon
  }
};
