const expect = require('chai').expect;
const { inflateSync } = require('zlib');

const { encode, generateUrl } = require('../../lib/diagram-encoder');

describe('# [unit-test]: diagram-encoder.js', () => {
    describe('## [function]: encode()', () => {
        it('* encoded data is able to decode.', () => {
            const testFunc = () => {
                const expected = '@startuml\nBob -> Alice : hello\n@enduml';

                const encoded = encode(expected);
                const deflated = Buffer.from(encoded, "base64url");
                const actual = inflateSync(deflated).toString();

                expect(actual).to.be.equal(expected);
            }
            expect(testFunc).not.to.Throw();
        });
    });
    describe('## [function]: generateUrl()', () => {
        it('* must start format like  <entry-point>/<lang>/<format>/', () => {
            const actual = generateUrl('https://kroki.io', 'graphviz', 'svg', 'digraph G {Hello->World}');
            const expected = 'https://kroki.io/graphviz/svg/';

            expect(actual).to.be.a('string');
            expect(actual.startsWith(expected)).to.be.true;
        });
        it('* must endwith <encoded>', () => {
            const actual = generateUrl('https://kroki.io', 'graphviz', 'svg', 'digraph G {Hello->World}');
            const expected = encode('digraph G {Hello->World}');

            expect(actual).to.be.a('string');
            expect(actual.endsWith(expected)).to.be.true;
        });
        [1, '', null, undefined].forEach(test => {
            it(`* [exception] throws when entry-point is, non-string object, empty string, null or undefined. Test: ${test}`, () => {
                const testFunction = () => {
                    let _ = generateUrl(test, 'graphviz', 'svg', 'digraph G {Hello->World}');
                }
                expect(testFunction).throw();
            });
            it(`* [exception] throws when lang is, non-string object, empty string, null or undefined. Test: ${test}`, () => {
                const testFunction = () => {
                    let _ = generateUrl('https://kroki.io', test, 'svg', 'digraph G {Hello->World}');
                }
                expect(testFunction).throw();
            });
            it(`* [exception] throws when imgType is, non-string object, empty string, null or undefined. Test: ${test}`, () => {
                const testFunction = () => {
                    let _ = generateUrl('https://kroki.io', 'graphviz', test, 'digraph G {Hello->World}');
                }
                expect(testFunction).throw();
            });
            it(`* [exception] throws when diagram is, non-string object, empty string, null or undefined. Test: ${test}`, () => {
                const testFunction = () => {
                    let _ = generateUrl('https://kroki.io', 'graphviz', 'svg', diagram);
                }
                expect(testFunction).throw();
            });
        });
        it('* [exception] throws when lang is unsupported lang', () => {
            const testFunction = () => {
                let _ = generateUrl('https://kroki.io', 'graphviz123', 'svg', 'digraph G {Hello->World}');
            }
            expect(testFunction).throw();
        });
        it('* [exception] throws when imgType is unsupported imgType', () => {
            const testFunction = () => {
                let _ = generateUrl('https://kroki.io', 'graphviz', 'svg123', 'digraph G {Hello->World}');
            }
            expect(testFunction).throw();
        });
    });
});
