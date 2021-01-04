"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.getRandomInt = void 0;
var getRandomInt = function (min, max) {
    return Math.floor(Math.random() * (max - min)) + min;
};
exports.getRandomInt = getRandomInt;
