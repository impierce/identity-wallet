import { z } from 'zod/v4';

// reference: https://eur-lex.europa.eu/eli/reg_impl/2024/2977/oj

// TODO: only allow values as defined in ISO 3166-1 alpha-2
const countryCode = z.string().length(2).uppercase();

export const residence = z.object({
  resident_address: z.string().optional(),
  resident_country: countryCode.optional(),
  resident_state: z.string().optional(),
  resident_city: z.string().optional(),
  resident_postal_code: z.string().optional(),
  resident_street: z.string().optional(),
  resident_house_number: z.string().optional(),
});

// const sex = {
//     0: 'not known',
//     1: 'male',
//     2: 'female',
//     3: 'other',
//     4: 'inter',
//     5: 'diverse',
//     6: 'open',
//     9: 'not applicable',
// }

export const naturalPerson = z.object({
  // mandatory
  family_name: z.string(),
  given_name: z.string(),
  birth_date: z.iso.date(),
  birth_place: countryCode, // TODO: or arbitrary string?
  nationality: z.array(countryCode).min(1),

  // optional
  family_name_birth: z.string().optional(),
  sex: z
    .number()
    .refine((val) => [0, 1, 2, 3, 4, 5, 6, 9].includes(val))
    .optional(),
});
