"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.argsChecker = void 0;
var argsChecker = function (commands, argsNumber, minimalArgs) {
    var checking = {
        commandData: commands,
        checking: {
            requiredArgs: argsNumber,
            receivedArgs: commands.arguments.length,
            checkStatus: commands.arguments.length >= minimalArgs
        }
    };
    if (!checking.checking.checkStatus) {
        throw Error("Expected " + checking.checking.requiredArgs + " arguments and got " + checking.checking.receivedArgs);
    }
    return checking;
};
exports.argsChecker = argsChecker;
