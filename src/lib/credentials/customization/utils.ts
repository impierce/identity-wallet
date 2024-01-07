import Bank from '~icons/ph/bank-light';
import Car from '~icons/ph/car-light';
import Certificate from '~icons/ph/certificate-light';
import EnvelopeSimple from '~icons/ph/envelope-simple-light';
import File from '~icons/ph/file-light';
import House from '~icons/ph/house-light';
import Leaf from '~icons/ph/leaf-light';
import Question from '~icons/ph/question-light';
import User from '~icons/ph/user-light';

export const icons = {
  Bank: Bank,
  EnvelopeSimple: EnvelopeSimple,
  House: House,
  Leaf: Leaf,
  User: User,
  Car: Car,
  Question: Question,
  Certificate: Certificate,
  File: File,
} as const;

export type Icon = keyof typeof icons;

/**
 * No dynamic class names, therefore explicitly defined background colors.
 * See: https://tailwindcss.com/docs/content-configuration#dynamic-class-names
 */
export const colors: Array<string> = [
  'bg-credentials-0',
  'bg-credentials-1',
  'bg-credentials-2',
  'bg-credentials-3',
  'bg-credentials-4',
  'bg-credentials-5',
  'bg-credentials-6',
  'bg-credentials-7',
  'bg-white',
];
