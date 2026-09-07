import iso3166 from './iso-3166-1_alpha-2.json';

// source: https://github.com/lukes/ISO-3166-Countries-with-Regional-Codes/blob/v10.0/slim-2/slim-2.json

export type Country = {
  code: string;
  name: string;
};

const countries: Country[] = iso3166.map((i) => ({ code: i['alpha-2'], name: i.name }));

countries.find((c) => c.code === 'GB')!.name = 'United Kingdom';
countries.find((c) => c.code === 'NL')!.name = 'Netherlands';

export default countries;
