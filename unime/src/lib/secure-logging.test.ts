import { sanitizeStringify } from './secure-logging';

describe('secure-logging', () => {
  test('Check if secure logged', () => {
    let credentialObj = {
      name: 'John doe',
      age: 44,
      password: 'please_dont_log',
    };

    let sampleStr = JSON.stringify(credentialObj);

    let secureStr = sanitizeStringify(credentialObj);

    console.log(secureStr);

    expect(sampleStr).toContain('please_dont_log');
    expect(secureStr).not.toContain('please_dont_log');
  });

  test('Check if recursively works', () => {
    let pw1 = 'please_dont_log';
    let pw2 = 'please_dont_log_as_well';
    let pw3 = 'dont_log';

    let credentialObj = {
      name: 'John doe',
      age: 44,
      password: pw1,
      data: {
        password: pw2,
        credentials: {
          password: pw3,
        },
      },
    };

    let sampleStr = JSON.stringify(credentialObj);
    let secureStr = sanitizeStringify(credentialObj);

    expect(secureStr).not.toContain(pw1);
    expect(secureStr).not.toContain(pw2);
    expect(secureStr).not.toContain(pw3);

    expect(sampleStr).toContain(pw1);
    expect(sampleStr).toContain(pw2);
    expect(sampleStr).toContain(pw3);
  });
});
