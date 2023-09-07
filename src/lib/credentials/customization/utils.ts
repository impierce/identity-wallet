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
  File: File
};

/**
 * No dynamic class names, therefore explicitly defined background colors.
 * See: https://tailwindcss.com/docs/content-configuration#dynamic-class-names
 */
export const colors: Array<string> = [
  'bg-[#D7E0CC]', // 0: green
  'bg-[#E3D1B6]', // 1: orange
  'bg-[#AFCBDD]', // 2: blue
  'bg-[#ACB7E2]', // 3: purple
  'bg-[#C6D3D8]', // 4: silver
  'bg-[#DDDCF1]', // 5: lavender
  'bg-[#D9CADD]', // 6: pink
  'bg-[#C9D9D0]', // 7: green
  'bg-white' // 8: white
];
