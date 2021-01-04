"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.joinArgs = void 0;
var joinArgs = function (args) {
    var argsAsString = "";
    for (var i = 0; i < args.length; i++) {
        argsAsString += args[i] + " ";
    }
    return argsAsString;
};
exports.joinArgs = joinArgs;
