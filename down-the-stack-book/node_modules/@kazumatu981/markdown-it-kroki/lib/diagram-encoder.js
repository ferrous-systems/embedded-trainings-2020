'use strict';

const { deflateSync } = require('zlib');
const contract = require('./contract');
const support = require('./support');

function encode(diagram) {
    return deflateSync(diagram, { level: 9 }).toString('base64url');
}
function generateUrl(entrypoint, lang, imgType, diagram) {

    contract.toNonEmptyString(entrypoint, '\'entrypoint\' must be non-empty string.');
    contract.toNonEmptyString(lang, '\'lang\' must be non-empty string.');
    contract.toNonEmptyString(imgType, '\'imgType\' must be non-empty string.');
    contract.toNonEmptyString(diagram, '\'diagram\' must be non-empty string.');
    contract.toTrue(support.languageSupports(lang), 'Not Supported Diagram Language.');
    contract.toTrue(support.imageFormatSupports(imgType), 'Not Supported Image Type.');

    const api = `${lang}/${imgType}/${encode(diagram)}`;

    return entrypoint.endsWith('/') ?
        `${entrypoint}${api}` : `${entrypoint}/${api}`;
}

module.exports = {
    encode, generateUrl
};
