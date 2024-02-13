import { sanitizeStringify } from './sensitive-logging';

describe('sensitive-logging', () => {
  test('Check if sensitive data is not logged', () => {
    let credentialObj = {
      name: 'John doe',
      age: 44,
      password: 'please_dont_log',
    };

    let sensitiveStr = sanitizeStringify(credentialObj);

    // Sample after because it shouldn't modify the passed object.
    let sampleStr = JSON.stringify(credentialObj);


    console.log(sensitiveStr);

    expect(sampleStr).toContain('please_dont_log');
    expect(sensitiveStr).not.toContain('please_dont_log');
  });

  test('Check if sensitive objects recursively is not logged', () => {
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

    let sensitiveStr = sanitizeStringify(credentialObj);

    // Sample after because it shouldn't modify the passed object.
    let sampleStr = JSON.stringify(credentialObj);

    expect(sensitiveStr).not.toContain(pw1);
    expect(sensitiveStr).not.toContain(pw2);
    expect(sensitiveStr).not.toContain(pw3);

    expect(sampleStr).toContain(pw1);
    expect(sampleStr).toContain(pw2);
    expect(sampleStr).toContain(pw3);
  });
});
