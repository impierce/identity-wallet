import type { TranslationFunctions } from '$i18n/i18n-types';
import { z } from 'zod';

// reference: https://eur-lex.europa.eu/eli/reg_impl/2024/2977/oj

// TODO: only allow values as defined in ISO 3166-1 alpha-2
const countryCode = z.string().length(2);

export const residence = (LL: TranslationFunctions) =>
  z.object({
    resident_address: z.string().optional(),
    resident_country: countryCode.min(1, LL.ADD_CREDENTIALS.VALUE_REQUIRED()),
    resident_state: z.string().optional(),
    resident_city: z.string().optional(),
    resident_postal_code: z.string().optional(),
    resident_street: z.string().optional(),
    resident_house_number: z.string().optional(),
  });

export const naturalPerson = (LL: TranslationFunctions) =>
  z.object({
    // mandatory
    family_name: z.string({ message: LL.ADD_CREDENTIALS.VALUE_REQUIRED() }),
    given_name: z.string(),
    // birth_date: z.iso.date(), // TODO: available in zod/v4
    birth_date: z.date(),
    birth_place: countryCode, // TODO: should be arbitrary string?
    nationality: z.array(countryCode).min(1, 'At least one nationality is required'),

    // optional
    family_name_birth: z.string().optional(),
    // const sex = {
    //   0: 'not known',
    //   1: 'male',
    //   2: 'female',
    //   3: 'other',
    //   4: 'inter',
    //   5: 'diverse',
    //   6: 'open',
    //   9: 'not applicable',
    // };
    sex: z
      .number()
      .refine((val) => [0, 1, 2, 3, 4, 5, 6, 9].includes(val))
      .optional(),
  });
